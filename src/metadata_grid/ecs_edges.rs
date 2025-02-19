use bevy_ecs::prelude::*;
use std::collections::HashMap;

use super::{components::EquipmentId, MetadataGrid};

#[derive(Resource, Default)]
pub struct EdgeRegistry(pub HashMap<String, Entity>);

pub fn spawn_edge(
    commands: &mut Commands,
    edge_registry: &mut EdgeRegistry,
    equipment_id: String,
) -> Entity {
    let entity = commands.spawn(EquipmentId(equipment_id.clone())).id();
    edge_registry.0.insert(equipment_id, entity);
    entity
}

pub fn find_edge_by_equipment_id(
    bus_node_registry: &EdgeRegistry,
    equipment_id: &str,
) -> Option<Entity> {
    bus_node_registry.0.get(equipment_id).copied()
}

pub fn load_edges_from_metadata(
    mut commands: Commands,
    mut bus_node_registry: ResMut<EdgeRegistry>,
    grid: &MetadataGrid,
) {
    for node in &grid.bus_nodes {
        spawn_edge(
            &mut commands,
            &mut bus_node_registry,
            node.equipment_id.clone(),
        );
    }
}
