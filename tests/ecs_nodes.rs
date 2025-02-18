use argus_mapping::*;
use bevy_ecs::prelude::*;
use serde_json::from_str;

// Test helper to create a test app with the required systems and resources
fn setup_test_app() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<NodeRegistry>();
    let schedule = Schedule::default();
    (world, schedule)
}

// Helper to check node creation results
fn check_node_creation(
    world: &World,
    node_registry: &NodeRegistry,
    equipment_id: &str,
    expected_exists: bool,
) {
    let entity = find_node_by_equipment_id(node_registry, equipment_id);
    assert_eq!(
        entity.is_some(),
        expected_exists,
        "Node existence check failed for {}",
        equipment_id
    );

    if let Some(entity) = entity {
        let equipment = world.get::<EquipmentId>(entity);
        assert!(
            equipment.is_some(),
            "Entity {} should have EquipmentId component",
            equipment_id
        );
        assert_eq!(
            equipment.unwrap().0,
            equipment_id,
            "Entity {} has incorrect equipment_id",
            equipment_id
        );
    }
}

// Test helper pour vérifier l'existence d'un nœud
fn check_node_exists(world: &World, node_registry: &NodeRegistry, equipment_id: &str) {
    let entity = find_node_by_equipment_id(node_registry, equipment_id);
    assert!(entity.is_some(), "Node {} should exist", equipment_id);

    if let Some(entity) = entity {
        let equipment = world.get::<EquipmentId>(entity);
        assert!(
            equipment.is_some(),
            "Entity {} should have EquipmentId component",
            equipment_id
        );
        assert_eq!(
            equipment.unwrap().0,
            equipment_id,
            "Entity {} has incorrect equipment_id",
            equipment_id
        );
    }
}

#[test]
fn test_spawn_single_node() {
    let (mut world, _) = setup_test_app();
    let mut node_registry = NodeRegistry::default();

    let equipment_id = "test_node_1".to_string();
    let entity = world.spawn(EquipmentId(equipment_id.clone())).id();
    node_registry.0.insert(equipment_id.clone(), entity);

    check_node_creation(&world, &node_registry, &equipment_id, true);
}

#[test]
fn test_find_nonexistent_node() {
    let node_registry = NodeRegistry::default();
    let result = find_node_by_equipment_id(&node_registry, "nonexistent");
    assert!(result.is_none(), "Should return None for nonexistent node");
}

#[test]
fn test_load_multiple_nodes() {
    let (mut world, _) = setup_test_app();
    let mut node_registry = NodeRegistry::default();

    // Create test metadata grid
    let grid = MetadataGrid {
        nodes: vec![
            Node {
                equipment_id: "node1".to_string(),
                ..Default::default()
            },
            Node {
                equipment_id: "node2".to_string(),
                ..Default::default()
            },
            Node {
                equipment_id: "node3".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Load nodes
    for node in &grid.nodes {
        let entity = world.spawn(EquipmentId(node.equipment_id.clone())).id();
        node_registry.0.insert(node.equipment_id.clone(), entity);
    }

    // Verify all nodes were created correctly
    for node in &grid.nodes {
        check_node_creation(&world, &node_registry, &node.equipment_id, true);
    }
}

#[test]
fn test_node_uniqueness() {
    let (mut world, _) = setup_test_app();
    let mut node_registry = NodeRegistry::default();

    // Spawn same node twice
    let equipment_id = "duplicate_node".to_string();

    let entity1 = world.spawn(EquipmentId(equipment_id.clone())).id();
    node_registry.0.insert(equipment_id.clone(), entity1);

    let entity2 = world.spawn(EquipmentId(equipment_id.clone())).id();
    node_registry.0.insert(equipment_id.clone(), entity2);

    // Verify only the latest entity is registered
    let registered_entity = find_node_by_equipment_id(&node_registry, &equipment_id)
        .expect("Node should exist in registry");
    assert_eq!(
        registered_entity, entity2,
        "Registry should contain the most recently spawned entity"
    );
    assert_ne!(
        registered_entity, entity1,
        "Registry should not contain the first spawned entity"
    );
}

#[test]
fn test_load_empty_grid() {
    let (mut world, _) = setup_test_app();
    let mut node_registry = NodeRegistry::default();
    let grid = MetadataGrid::default();

    // Load nodes (should do nothing for empty grid)
    for node in &grid.nodes {
        let entity = world.spawn(EquipmentId(node.equipment_id.clone())).id();
        node_registry.0.insert(node.equipment_id.clone(), entity);
    }

    assert!(
        node_registry.0.is_empty(),
        "Node registry should be empty for empty grid"
    );
}

#[test]
fn test_metadata_grid_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let mut world = World::new();
    world.init_resource::<NodeRegistry>();

    // Charger les données JSON
    let json_content = std::fs::read_to_string("tests/data/simple-eu.json")?;
    let grid: MetadataGrid = from_str(&json_content)?;

    // Création des nœuds
    let mut node_registry = NodeRegistry::default();

    // Spawner les nœuds
    for node in &grid.nodes {
        let entity = world.spawn(EquipmentId(node.equipment_id.clone())).id();
        node_registry.0.insert(node.equipment_id.clone(), entity);
    }

    // Vérifications

    // 1. Vérifier le nombre total de nœuds
    assert_eq!(
        node_registry.0.len(),
        grid.nodes.len(),
        "Number of registered nodes doesn't match input data"
    );

    // 2. Vérifier quelques nœuds spécifiques
    check_node_exists(&world, &node_registry, "VL1");
    check_node_exists(&world, &node_registry, "VL10");
    check_node_exists(&world, &node_registry, "VL100");

    // 3. Vérifier qu'un nœud inexistant n'existe pas
    let nonexistent = find_node_by_equipment_id(&node_registry, "NONEXISTENT");
    assert!(
        nonexistent.is_none(),
        "Nonexistent node should not be found"
    );

    // 4. Vérifier que les entités ont les bons composants
    for (equipment_id, entity) in &node_registry.0 {
        let equipment = world
            .get::<EquipmentId>(*entity)
            .expect("Entity should have EquipmentId component");
        assert_eq!(
            &equipment.0, equipment_id,
            "Entity should have correct equipment_id"
        );
    }

    Ok(())
}

// Test le comportement avec un dataset vide
#[test]
fn test_empty_metadata_grid() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = World::new();
    world.init_resource::<NodeRegistry>();

    let _empty_grid = MetadataGrid::default();
    let node_registry = NodeRegistry::default();

    // Vérifier qu'aucun nœud n'est créé
    assert!(
        node_registry.0.is_empty(),
        "Registry should be empty for empty grid"
    );

    Ok(())
}

// Test la gestion des doublons
#[test]
fn test_duplicate_nodes() {
    let mut world = World::new();
    world.init_resource::<NodeRegistry>();
    let mut node_registry = NodeRegistry::default();

    // Créer deux fois le même nœud
    let duplicate_id = "DUPLICATE_NODE".to_string();

    let entity1 = world.spawn(EquipmentId(duplicate_id.clone())).id();
    node_registry.0.insert(duplicate_id.clone(), entity1);

    let entity2 = world.spawn(EquipmentId(duplicate_id.clone())).id();
    node_registry.0.insert(duplicate_id.clone(), entity2);

    // Vérifier que seule la dernière entité est enregistrée
    let registered_entity = find_node_by_equipment_id(&node_registry, &duplicate_id)
        .expect("Node should exist in registry");
    assert_eq!(
        registered_entity, entity2,
        "Registry should contain the most recently spawned entity"
    );
    assert_ne!(
        registered_entity, entity1,
        "Registry should not contain the first spawned entity"
    );
}
