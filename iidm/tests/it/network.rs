use iidm::Network;
use iidm::Updatable;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

mod test_data {
    use super::*;

    pub const VALID_NETWORK_JSON: &str = r#"{
        "version": "1.12",
        "id": "sim1",
        "caseDate": "2013-01-15T18:45:00.000+01:00",
        "forecastDistance": 0,
        "sourceFormat": "test",
        "minimumValidationLevel": "STEADY_STATE_HYPOTHESIS",
        "substations": [],
        "lines": []
    }"#;

    pub fn create_default_network() -> Network {
        serde_json::from_str(VALID_NETWORK_JSON).unwrap()
    }

    pub fn assert_default_values(network: &Network) {
        assert_eq!(network.version, "1.12");
        assert_eq!(network.id, "sim1");
        assert_eq!(
            network.case_date.format(DATETIME_FORMAT).to_string(),
            "2013-01-15T18:45:00.000+01:00"
        );
        assert_eq!(network.forecast_distance, 0);
        assert_eq!(network.source_format, "test");
        assert_eq!(network.minimum_validation_level, "STEADY_STATE_HYPOTHESIS");
        assert!(network.substations.is_empty());
        assert!(network.lines.is_empty());
    }
}

mod serialization {
    use super::*;
    use test_data::*;

    #[test]
    fn test_deserialize_from_json() {
        let network: Network = serde_json::from_str(VALID_NETWORK_JSON).unwrap();
        assert_default_values(&network);
    }

    #[test]
    fn test_serialize_to_json() {
        let network = create_default_network();
        let json = serde_json::to_string(&network).unwrap();
        let deserialized: Network = serde_json::from_str(&json).unwrap();
        assert_default_values(&deserialized);
    }
}

mod updates {
    use std::str::FromStr;

    use super::*;
    use chrono::DateTime;
    use iidm::NetworkUpdate;
    use test_data::*;

    #[test]
    fn test_update_basic_fields() {
        let mut network = create_default_network();
        network.update(NetworkUpdate {
            case_date: Some(DateTime::from_str("2024-02-21T10:00:00.000+01:00").unwrap()),
            forecast_distance: Some(1),
            source_format: Some("updated".to_string()),
            minimum_validation_level: Some("EQUIPMENT".to_string()),
            ..Default::default()
        });

        assert_eq!(
            network.case_date.format(DATETIME_FORMAT).to_string(),
            "2024-02-21T10:00:00.000+01:00"
        );
        assert_eq!(network.forecast_distance, 1);
        assert_eq!(network.source_format, "updated");
        assert_eq!(network.minimum_validation_level, "EQUIPMENT");
        // Version should not be modifiable
        assert_eq!(network.version, "1.12");
    }

    #[test]
    fn test_update_with_empty_update() {
        let mut network = create_default_network();
        let original = create_default_network();

        network.update(NetworkUpdate::default());

        assert_eq!(
            serde_json::to_value(&network).unwrap(),
            serde_json::to_value(&original).unwrap()
        );
    }
}

mod json_updates {
    use super::*;
    use test_data::*;

    #[test]
    fn test_json_update_with_invalid_json() {
        let mut network = create_default_network();
        assert!(network.update_from_json("invalid json").is_err());
    }

    #[test]
    fn test_json_update_with_empty_json() {
        let mut network = create_default_network();
        let original = create_default_network();
        network.update_from_json("{}").unwrap();
        assert_eq!(
            serde_json::to_value(&network).unwrap(),
            serde_json::to_value(&original).unwrap()
        );
    }

    #[test]
    fn test_json_update_with_valid_fields() {
        let mut network = create_default_network();
        network
            .update_from_json(
                r#"{
                "caseDate": "2024-02-21T10:00:00.000+01:00",
                "forecastDistance": 2
            }"#,
            )
            .unwrap();

        assert_eq!(
            network.case_date.format(DATETIME_FORMAT).to_string(),
            "2024-02-21T10:00:00.000+01:00"
        );
        assert_eq!(network.forecast_distance, 2);
    }
}

#[cfg(test)]
mod integration_tests {
    use std::str::FromStr;

    use bevy_ecs::{
        entity::Entity,
        event::Events,
        schedule::Schedule,
        system::Commands,
        world::{CommandQueue, World},
    };
    use chrono::DateTime;
    use iidm::{handle_update_events, AssetRegistry, Network, NetworkUpdate, UpdateEvent};

    use crate::it::network::DATETIME_FORMAT;

    #[test]
    fn test_register_and_update_network() {
        // Setup
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.init_resource::<Events<UpdateEvent<Network>>>();
        schedule.add_systems(handle_update_events::<Network>);

        let mut registry = AssetRegistry::default();

        // Create initial network through registry
        let initial_network = Network {
            version: "1.12".to_string(),
            id: "test_network".to_string(),
            case_date: DateTime::from_str("2024-02-21T10:00:00.000+01:00").unwrap(),
            forecast_distance: 0,
            source_format: "test".to_string(),
            minimum_validation_level: "STEADY_STATE_HYPOTHESIS".to_string(),
            ..Default::default()
        };

        let entity = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            registry.add_component(&mut commands, "test_network", initial_network);
            queue.apply(&mut world);
            registry.find("test_network").unwrap()
        };

        // Create and send update event
        let update_event = UpdateEvent {
            entity,
            updates: NetworkUpdate {
                case_date: Some(DateTime::from_str("2024-02-21T11:00:00.000+01:00").unwrap()),
                forecast_distance: Some(1),
                ..Default::default()
            },
        };

        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Network>>>();
        event_writer.send(update_event);

        // Run the system
        schedule.run(&mut world);

        // Verify the network was updated correctly
        let network = world.entity(entity).get::<Network>().unwrap();
        assert_eq!(
            network.case_date.format(DATETIME_FORMAT).to_string(),
            "2024-02-21T11:00:00.000+01:00"
        );
        assert_eq!(network.forecast_distance, 1);
        assert_eq!(network.version, "1.12"); // Version should remain unchanged
    }

    #[test]
    fn test_multiple_registered_networks_update() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.init_resource::<Events<UpdateEvent<Network>>>();
        schedule.add_systems(handle_update_events::<Network>);

        let mut registry = AssetRegistry::default();

        // Create multiple networks
        let networks = vec![
            (
                "network1",
                Network {
                    version: "1.12".to_string(),
                    id: "network1".to_string(),
                    case_date: DateTime::from_str("2024-02-21T10:00:00.000+01:00").unwrap(),
                    forecast_distance: 0,
                    source_format: "test1".to_string(),
                    minimum_validation_level: "STEADY_STATE_HYPOTHESIS".to_string(),
                    ..Default::default()
                },
            ),
            (
                "network2",
                Network {
                    version: "1.12".to_string(),
                    id: "network2".to_string(),
                    case_date: DateTime::from_str("2024-02-21T10:00:00.000+01:00").unwrap(),
                    forecast_distance: 1,
                    source_format: "test2".to_string(),
                    minimum_validation_level: "STEADY_STATE_HYPOTHESIS".to_string(),
                    ..Default::default()
                },
            ),
        ];

        // Register all networks
        let entities: Vec<(String, Entity)> = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);

            let entities = networks
                .iter()
                .map(|(id, network)| {
                    registry.add_component(&mut commands, id.to_string(), network.clone());
                    (id.to_string(), registry.find(*id).unwrap())
                })
                .collect();

            queue.apply(&mut world);
            entities
        };

        // Update each network
        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Network>>>();
        for (_, entity) in &entities {
            event_writer.send(UpdateEvent {
                entity: *entity,
                updates: NetworkUpdate {
                    source_format: Some("updated".to_string()),
                    minimum_validation_level: Some("EQUIPMENT".to_string()),
                    ..Default::default()
                },
            });
        }

        schedule.run(&mut world);

        // Verify updates
        for (id, entity) in entities {
            let network = world.entity(entity).get::<Network>().unwrap();
            assert_eq!(
                network.source_format, "updated",
                "Network {} not updated correctly",
                id
            );
            assert_eq!(
                network.minimum_validation_level, "EQUIPMENT",
                "Network {} not updated correctly",
                id
            );
        }
    }

    #[test]
    fn test_update_removed_registered_network() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.init_resource::<Events<UpdateEvent<Network>>>();
        schedule.add_systems(handle_update_events::<Network>);

        let mut registry = AssetRegistry::default();

        // Create and register a network
        let network = Network {
            version: "1.12".to_string(),
            id: "test_network".to_string(),
            case_date: DateTime::from_str("2024-02-21T10:00:00.000+01:00").unwrap(),
            forecast_distance: 0,
            source_format: "test".to_string(),
            minimum_validation_level: "STEADY_STATE_HYPOTHESIS".to_string(),
            ..Default::default()
        };

        let entity = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            registry.add_component(&mut commands, "test_network", network);
            queue.apply(&mut world);
            registry.find("test_network").unwrap()
        };

        // Remove the entity
        {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            commands.entity(entity).despawn();
            queue.apply(&mut world);
        }

        // Try to update the removed entity
        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Network>>>();
        event_writer.send(UpdateEvent {
            entity,
            updates: NetworkUpdate {
                ..Default::default()
            },
        });

        // System should handle this gracefully
        schedule.run(&mut world);

        // Verify the entity no longer exists
        assert!(!world.entities().contains(entity));
    }
}
