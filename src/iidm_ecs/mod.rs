mod ecs_switch;

use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug)]
pub struct Identifiable(pub String);

#[derive(Resource, Default)]
pub struct PhysicalAssetRegistry(pub HashMap<String, Entity>);

pub fn spawn_physical_asset(
    commands: &mut Commands,
    registery: &mut PhysicalAssetRegistry,
    id: String,
) -> Entity {
    let entity = commands.spawn(Identifiable(id.clone())).id();
    registery.0.insert(id, entity);
    entity
}

pub fn find_physical_asset_by_id(registery: &PhysicalAssetRegistry, id: &str) -> Option<Entity> {
    registery.0.get(id).copied()
}
