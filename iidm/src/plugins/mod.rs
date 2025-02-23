use std::path::PrefixComponent;

use bevy_ecs::prelude::*;

use crate::{AssetRegistry, Identifiable, Updatable};

#[derive(Event)]
pub struct UpdateEvent<T: Updatable>
where
    T: 'static,
    T::Event: Send + Sync,
{
    pub id: String,
    pub update: T::Event,
}

pub fn handle_update_events<T: Component + Updatable>(
    mut update_events: EventReader<UpdateEvent<T>>,
    registery: Res<AssetRegistry>,
    mut query: Query<&mut T>,
) where
    T: 'static,
    T::Event: Send + Sync + Clone,
{
    for UpdateEvent { id, update } in update_events.read() {
        if let Some(entity) = registery.find(id) {
            if let Ok(mut component) = query.get_mut(entity) {
                component.update(update.clone());
            }
        }
    }
}

#[derive(Event)]
pub struct RegisterEvent<T: Component + Identifiable>
where
    T: 'static,
{
    pub id: String,
    pub component: T,
}

pub fn handle_register_events<T: Component + Identifiable + Clone>(
    mut register_events: EventReader<RegisterEvent<T>>,
    mut commands: Commands,
    mut registery: ResMut<AssetRegistry>,
) where
    T: 'static,
{
    for RegisterEvent { id, component } in register_events.read() {
        dbg!("{:?}", id);
        registery.add_component(&mut commands, id, component.clone());
    }
}
