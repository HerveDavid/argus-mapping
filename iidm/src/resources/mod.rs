use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::Identifiable;

#[derive(Component, Debug)]
pub struct Id(pub String);

#[derive(Resource, Default)]
pub struct PhysicalAssetRegistry(pub HashMap<String, Entity>);

impl PhysicalAssetRegistry {
    pub fn spawn_physical_asset<S>(&mut self, commands: &mut Commands, id: S) -> Entity where S: Into<String> {
        let id = id.into();
        let entity = commands.spawn(Id(id.clone())).id();
        self.0.insert(id, entity);
        entity
    }

    pub fn spawn_identifiable<C>(&mut self, commands: &mut Commands, component: C) -> Entity
    where
        C: Component + Identifiable,
    {
        let id = component.id();
        match self.find_physical_asset_by_id(&id) {
            Some(entity) => {
                commands.entity(entity).insert(component);
                entity
            }
            None => {
                 self.spawn_physical_asset(commands, id)
            }
        }
    }

    pub fn insert_component<C, S>(
        &mut self,
        commands: &mut Commands,
        id: S,
        component: C,
    ) 
    where
        C: Component,
        S: Into<String>,
    {
        let id: String = id.into();
        match self.find_physical_asset_by_id(id.clone()) {
            Some(entity)=> {
                commands.entity(entity).insert(component);
            }
            None => {
                let entity =  self.spawn_physical_asset(commands, id);
                commands.entity(entity).insert(component);
            }
        }
    }

    pub fn find_physical_asset_by_id<S>(&self, id: S) -> Option<Entity>
    where
        S: Into<String>,
    {
        self.0.get(&id.into()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::CommandQueue;

    #[test]
    fn test_spawn_physical_asset() {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();

        let test_id = "test_asset_1".to_string();
        {
            let mut commands = Commands::new(&mut queue, &world);
            let entity = registry.spawn_physical_asset(&mut commands, test_id.clone());

            queue.apply(&mut world);

            let identifiable = world.get::<Id>(entity);
            assert!(identifiable.is_some());
            assert_eq!(identifiable.unwrap().0, test_id);

            assert!(registry.0.contains_key(&test_id));
            assert_eq!(registry.0.get(&test_id), Some(&entity));
        }
    }

    #[test]
    fn test_find_physical_asset_by_id() {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();

        // Créer plusieurs entités
        let id1 = "asset_1".to_string();
        let id2 = "asset_2".to_string();

        let (entity1, entity2) = {
            let mut commands = Commands::new(&mut queue, &world);
            let e1 = registry.spawn_physical_asset(&mut commands, id1.clone());
            let e2 = registry.spawn_physical_asset(&mut commands, id2.clone());
            (e1, e2)
        };

        queue.apply(&mut world);

        assert_eq!(registry.find_physical_asset_by_id(&id1), Some(entity1));
        assert_eq!(registry.find_physical_asset_by_id(&id2), Some(entity2));

        assert_eq!(registry.find_physical_asset_by_id("non_existent"), None);
    }

    #[test]
    fn test_multiple_assets_registration() {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();

        let entities: Vec<(String, Entity)> = {
            let mut commands = Commands::new(&mut queue, &world);
            (0..5)
                .map(|i| {
                    let id = format!("asset_{}", i);
                    let entity = registry.spawn_physical_asset(&mut commands, id.clone());
                    (id, entity)
                })
                .collect()
        };

        queue.apply(&mut world);

        for (id, entity) in entities {
            assert_eq!(registry.find_physical_asset_by_id(&id), Some(entity));

            let identifiable = world.get::<Id>(entity);
            assert!(identifiable.is_some());
            assert_eq!(identifiable.unwrap().0, id);
        }

        assert_eq!(registry.0.len(), 5);
    }
}
