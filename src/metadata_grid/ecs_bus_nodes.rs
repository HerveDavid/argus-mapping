use bevy_ecs::prelude::*;
use std::collections::HashMap;

use super::components::EquipmentId;
use super::MetadataGrid;

#[derive(Resource, Default)]
pub struct BusNodeRegistry(pub HashMap<String, Entity>);

pub fn spawn_bus_node(
    commands: &mut Commands,
    bus_node_registry: &mut BusNodeRegistry,
    equipment_id: String,
) -> Entity {
    let entity = commands.spawn(EquipmentId(equipment_id.clone())).id();
    bus_node_registry.0.insert(equipment_id, entity);
    entity
}

pub fn find_bus_node_by_equipment_id(
    bus_node_registry: &BusNodeRegistry,
    equipment_id: &str,
) -> Option<Entity> {
    bus_node_registry.0.get(equipment_id).copied()
}

pub fn load_bus_nodes_from_metadata(
    mut commands: Commands,
    mut bus_node_registry: ResMut<BusNodeRegistry>,
    grid: &MetadataGrid,
) {
    for node in &grid.bus_nodes {
        spawn_bus_node(
            &mut commands,
            &mut bus_node_registry,
            node.equipment_id.clone(),
        );
    }
}
