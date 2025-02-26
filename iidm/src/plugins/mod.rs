use bevy_ecs::prelude::*;

use crate::{AssetRegistry, Identifiable, Updatable};

#[derive(Event)]
pub struct UpdateEvent<T: Updatable>
where
    T: 'static,
    T::Updater: Send + Sync,
{
    pub id: String,
    pub updater: T::Updater,
}

#[derive(Event, Debug, Clone)]
pub struct EntityNotFoundEvent {
    pub id: String,
    pub error_type: ErrorType,
    pub component_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    EntityNotFound,
    ComponentNotFound,
}

pub fn handle_update_events<T: Component + Updatable>(
    mut update_events: EventReader<UpdateEvent<T>>,
    mut error_events: EventWriter<EntityNotFoundEvent>,
    registery: Res<AssetRegistry>,
    mut query: Query<&mut T>,
) where
    T: 'static,
    T::Updater: Send + Sync + Clone,
{
    for UpdateEvent {
        id,
        updater: update,
    } in update_events.read()
    {
        match registery.find(id) {
            Some(entity) => {
                match query.get_mut(entity) {
                    Ok(mut component) => {
                        component.update(update.clone());
                    }
                    Err(_) => {
                        // Component exists but has wrong type
                        error_events.send(EntityNotFoundEvent {
                            id: id.clone(),
                            error_type: ErrorType::ComponentNotFound,
                            component_type: std::any::type_name::<T>().to_string(),
                        });
                    }
                }
            }
            None => {
                // Entity with this ID doesn't exist
                error_events.send(EntityNotFoundEvent {
                    id: id.clone(),
                    error_type: ErrorType::EntityNotFound,
                    component_type: std::any::type_name::<T>().to_string(),
                });
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
        registery.add_component(&mut commands, id, component.clone());
    }
}
