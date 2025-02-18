use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::MetadataGrid;

use super::components::EquipmentId;

#[derive(Resource, Default)]
pub struct NodeRegistry(pub HashMap<String, Entity>);

pub fn spawn_node(
    commands: &mut Commands,
    node_registry: &mut NodeRegistry,
    equipment_id: String,
) -> Entity {
    let entity = commands.spawn(EquipmentId(equipment_id.clone())).id();
    node_registry.0.insert(equipment_id, entity);
    entity
}

pub fn find_node_by_equipment_id(
    node_registry: &NodeRegistry,
    equipment_id: &str,
) -> Option<Entity> {
    node_registry.0.get(equipment_id).copied()
}

pub fn load_nodes_from_metadata(
    mut commands: Commands,
    mut node_registry: ResMut<NodeRegistry>,
    grid: &MetadataGrid,
) {
    for node in &grid.nodes {
        spawn_node(&mut commands, &mut node_registry, node.equipment_id.clone());
    }
}
