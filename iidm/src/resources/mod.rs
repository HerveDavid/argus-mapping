use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// Unique identifier component for an entity
#[derive(Debug, Clone, Component)]
pub struct Id(String);

impl Id {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Registry for managing entities with unique identifiers
#[derive(Resource, Default)]
pub struct AssetRegistry {
    entities: HashMap<String, Entity>,
}

impl AssetRegistry {
    /// Creates a new entity with an ID and registers it
    pub fn register<S: Into<String>>(&mut self, commands: &mut Commands, id: S) -> Entity {
        let id = id.into();
        let entity = commands.spawn(Id::new(id.clone())).id();
        self.entities.insert(id, entity);
        entity
    }

    /// Finds an entity by its ID
    pub fn find<S: Into<String>>(&self, id: S) -> Option<Entity> {
        self.entities.get(&id.into()).copied()
    }

    /// Adds or updates a component on an entity, creating the entity if it doesn't exist
    pub fn add_component<S, C>(&mut self, commands: &mut Commands, id: S, component: C)
    where
        S: Into<String>,
        C: Component,
    {
        let id = id.into();
        let entity = self
            .find(&id)
            .unwrap_or_else(|| self.register(commands, id));

        commands.entity(entity).insert(component);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::{CommandQueue, World};

    #[test]
    fn test_id_creation() {
        let id = Id::new("test_entity");
        assert_eq!(id.value(), "test_entity");
    }

    #[test]
    fn test_register_new_entity() {
        let mut world = World::new();
        let mut registry = AssetRegistry {
            entities: HashMap::new(),
        };

        {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &mut world);
            registry.register(&mut commands, "test_entity");
            queue.apply(&mut world);
        }

        assert!(registry.entities.contains_key("test_entity"));
    }

    #[test]
    fn test_find_entity() {
        let mut world = World::new();
        let mut registry = AssetRegistry {
            entities: HashMap::new(),
        };

        {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &mut world);
            registry.register(&mut commands, "test_entity");
            queue.apply(&mut world);
        }

        let found = registry.find("test_entity");
        assert!(found.is_some());
    }

    #[test]
    fn test_find_nonexistent_entity() {
        let registry = AssetRegistry {
            entities: HashMap::new(),
        };
        let found = registry.find("nonexistent");
        assert!(found.is_none());
    }

    #[test]
    fn test_add_component() {
        let mut world = World::new();
        let mut registry = AssetRegistry {
            entities: HashMap::new(),
        };

        #[derive(Component, Debug)]
        struct TestComponent;

        {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &mut world);
            registry.add_component(&mut commands, "test_entity", TestComponent);
            queue.apply(&mut world);
        }

        let entity = registry.find("test_entity").unwrap();
        assert!(world.entity(entity).contains::<TestComponent>());
    }

    #[test]
    fn test_add_component_to_existing_entity() {
        let mut world = World::new();
        let mut registry = AssetRegistry {
            entities: HashMap::new(),
        };

        let entity = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &mut world);
            let entity = registry.register(&mut commands, "test_entity");
            queue.apply(&mut world);
            entity
        };

        #[derive(Component, Debug)]
        struct TestComponent;

        {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &mut world);
            registry.add_component(&mut commands, "test_entity", TestComponent);
            queue.apply(&mut world);
        }

        let found = registry.find("test_entity").unwrap();
        assert_eq!(entity, found);
        assert!(world.entity(entity).contains::<TestComponent>());
    }
}
