use bevy_ecs::{prelude::*, query};

use crate::iidm_json::Switch;

use super::{find_physical_asset_by_id, PhysicalAssetRegistry};

pub fn add_switch_component(commands: &mut Commands, entity: Entity, switch: Switch) {
    commands.entity(entity).insert(switch);
}

pub fn load_switchs_from_iidm(
    commands: &mut Commands,
    registery: ResMut<PhysicalAssetRegistry>,
    switchs: Vec<Switch>,
) {
    for switch in switchs {
        find_physical_asset_by_id(&registery, &switch.id).map(|e| {
            add_switch_component(commands, e, switch);
        });
    }
}

pub fn change_switch(mut query: Query<&mut Switch>, open: bool, entity: Entity) {
    if let Ok(mut switch) = query.get_mut(entity) {
        switch.open = open;
    }
}
