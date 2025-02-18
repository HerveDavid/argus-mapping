use argus_mapping::*;
use bevy_ecs::prelude::*;
use serde_json::from_str;

// Test helper to create a test app with the required systems and resources
fn setup_test_app() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<BusNodeRegistry>();
    let schedule = Schedule::default();
    (world, schedule)
}

// Helper to check node creation results
fn check_node_creation(
    world: &World,
    bus_node_registry: &BusNodeRegistry,
    equipment_id: &str,
    expected_exists: bool,
) {
    let entity = find_bus_node_by_equipment_id(bus_node_registry, equipment_id);
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
fn check_bus_node_exists(world: &World, bus_node_registry: &BusNodeRegistry, equipment_id: &str) {
    let entity = find_bus_node_by_equipment_id(bus_node_registry, equipment_id);
    assert!(entity.is_some(), "BusNode {} should exist", equipment_id);

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
fn test_spawn_single_bus_node() {
    let (mut world, _) = setup_test_app();
    let mut bus_node_registry = BusNodeRegistry::default();

    let equipment_id = "test_bus_node_1".to_string();
    let entity = world.spawn(EquipmentId(equipment_id.clone())).id();
    bus_node_registry.0.insert(equipment_id.clone(), entity);

    check_node_creation(&world, &bus_node_registry, &equipment_id, true);
}

#[test]
fn test_find_nonexistent_bus_node() {
    let bus_node_registry = BusNodeRegistry::default();
    let result = find_bus_node_by_equipment_id(&bus_node_registry, "nonexistent");
    assert!(result.is_none(), "Should return None for nonexistent node");
}

#[test]
fn test_load_multiple_bus_nodes() {
    let (mut world, _) = setup_test_app();
    let mut bus_node_registry = BusNodeRegistry::default();

    // Create test metadata grid
    let grid = MetadataGrid {
        bus_nodes: vec![
            BusNode {
                equipment_id: "node1".to_string(),
                ..Default::default()
            },
            BusNode {
                equipment_id: "node2".to_string(),
                ..Default::default()
            },
            BusNode {
                equipment_id: "node3".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Load nodes
    for bus_node in &grid.bus_nodes {
        let entity = world.spawn(EquipmentId(bus_node.equipment_id.clone())).id();
        bus_node_registry
            .0
            .insert(bus_node.equipment_id.clone(), entity);
    }

    // Verify all nodes were created correctly
    for bus_node in &grid.bus_nodes {
        check_node_creation(&world, &bus_node_registry, &bus_node.equipment_id, true);
    }
}

#[test]
fn test_bus_node_uniqueness() {
    let (mut world, _) = setup_test_app();
    let mut bus_node_registry = BusNodeRegistry::default();

    // Spawn same node twice
    let equipment_id = "duplicate_node".to_string();

    let entity1 = world.spawn(EquipmentId(equipment_id.clone())).id();
    bus_node_registry.0.insert(equipment_id.clone(), entity1);

    let entity2 = world.spawn(EquipmentId(equipment_id.clone())).id();
    bus_node_registry.0.insert(equipment_id.clone(), entity2);

    // Verify only the latest entity is registered
    let registered_entity = find_bus_node_by_equipment_id(&bus_node_registry, &equipment_id)
        .expect("BusNode should exist in registry");
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
    let mut bus_node_registry = BusNodeRegistry::default();
    let grid = MetadataGrid::default();

    // Load nodes (should do nothing for empty grid)
    for bus_node in &grid.bus_nodes {
        let entity = world.spawn(EquipmentId(bus_node.equipment_id.clone())).id();
        bus_node_registry
            .0
            .insert(bus_node.equipment_id.clone(), entity);
    }

    assert!(
        bus_node_registry.0.is_empty(),
        "BusNode registry should be empty for empty grid"
    );
}

#[test]
fn test_metadata_grid_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let mut world = World::new();
    world.init_resource::<BusNodeRegistry>();

    // Charger les données JSON
    let json_content = std::fs::read_to_string("tests/data/simple-eu.json")?;
    let grid: MetadataGrid = from_str(&json_content)?;

    // Création des nœuds
    let mut bus_node_registry = BusNodeRegistry::default();

    // Spawner les nœuds
    for bus_node in &grid.bus_nodes {
        let entity = world.spawn(EquipmentId(bus_node.equipment_id.clone())).id();
        bus_node_registry
            .0
            .insert(bus_node.equipment_id.clone(), entity);
    }

    // Vérifications

    // 1. Vérifier le nombre total de nœuds
    assert_eq!(
        bus_node_registry.0.len(),
        grid.bus_nodes.len(),
        "Number of registered bus nodes doesn't match input data"
    );

    // 2. Vérifier quelques nœuds spécifiques
    check_bus_node_exists(&world, &bus_node_registry, "VL1_0");
    check_bus_node_exists(&world, &bus_node_registry, "VL1_1");
    check_bus_node_exists(&world, &bus_node_registry, "VL10_1");

    // 3. Vérifier qu'un nœud inexistant n'existe pas
    let nonexistent = find_bus_node_by_equipment_id(&bus_node_registry, "NONEXISTENT");
    assert!(
        nonexistent.is_none(),
        "Nonexistent node should not be found"
    );

    // 4. Vérifier que les entités ont les bons composants
    for (equipment_id, entity) in &bus_node_registry.0 {
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
    world.init_resource::<BusNodeRegistry>();

    let _empty_grid = MetadataGrid::default();
    let node_registry = BusNodeRegistry::default();

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
    world.init_resource::<BusNodeRegistry>();
    let mut node_registry = BusNodeRegistry::default();

    // Créer deux fois le même nœud
    let duplicate_id = "DUPLICATE_NODE".to_string();

    let entity1 = world.spawn(EquipmentId(duplicate_id.clone())).id();
    node_registry.0.insert(duplicate_id.clone(), entity1);

    let entity2 = world.spawn(EquipmentId(duplicate_id.clone())).id();
    node_registry.0.insert(duplicate_id.clone(), entity2);

    // Vérifier que seule la dernière entité est enregistrée
    let registered_entity = find_bus_node_by_equipment_id(&node_registry, &duplicate_id)
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
