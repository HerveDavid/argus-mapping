use chrono::DateTime;
use iidm::*;
use std::str::FromStr;

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

    pub fn create_test_network() -> Network {
        Network {
            version: "1.0".to_string(),
            id: "test_network".to_string(),
            case_date: DateTime::from_str("2024-02-23T10:00:00.000+01:00").unwrap(),
            forecast_distance: 0,
            source_format: "test".to_string(),
            minimum_validation_level: "STEADY_STATE_HYPOTHESIS".to_string(),
            substations: vec![
                Substation {
                    id: "sub1".to_string(),
                    country: "FR".to_string(),
                    tso: "RTE".to_string(),
                    geographical_tags: vec!["region1".to_string()],
                    voltage_levels: vec![],
                    two_windings_transformers: vec![],
                },
                Substation {
                    id: "sub2".to_string(),
                    country: "FR".to_string(),
                    tso: "RTE".to_string(),
                    geographical_tags: vec!["region2".to_string()],
                    voltage_levels: vec![],
                    two_windings_transformers: vec![],
                },
            ],
            lines: vec![],
            three_windings_transformers: vec![],
            switches: vec![],
            shunt_compensators: vec![],
            static_var_compensators: vec![],
            dangling_lines: vec![],
            tie_lines: vec![],
            hvdc_lines: vec![],
        }
    }

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

mod identiable {
    use bevy_ecs::{event::Events, schedule::Schedule, world::World};
    use iidm::*;

    use super::*;

    #[test]
    fn test_network_register() {
        // Create a new world and schedule
        let mut world = World::new();
        let mut schedule = Schedule::default();

        // Initialize required resources
        world.init_resource::<AssetRegistry>();
        world.init_resource::<Events<RegisterEvent<Network>>>();
        world.init_resource::<Events<RegisterEvent<Substation>>>();

        // Add systems to schedule
        schedule.add_systems(handle_register_events::<Network>);
        schedule.add_systems(handle_register_events::<Substation>);

        // Create and register network
        let network = test_data::create_test_network();
        network.register(&mut world, &mut schedule);

        // Verify substations were registered
        let registry = world.resource::<AssetRegistry>();

        // Check if substations exist in registry
        assert!(registry.find("test_network").is_some());
        assert!(registry.find("sub1").is_some());
        assert!(registry.find("sub2").is_some());

        // Query for actual substation components
        let mut substation_query = world.query::<&Substation>();
        let substations: Vec<&Substation> = substation_query.iter(&world).collect();

        assert_eq!(substations.len(), 2);
        assert!(substations.iter().any(|s| s.id == "sub1"));
        assert!(substations.iter().any(|s| s.id == "sub2"));
    }

    #[test]
    fn test_network_register_empty() {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        world.init_resource::<AssetRegistry>();
        world.init_resource::<Events<RegisterEvent<Network>>>();
        world.init_resource::<Events<RegisterEvent<Substation>>>();
        schedule.add_systems(handle_register_events::<Network>);
        schedule.add_systems(handle_register_events::<Substation>);

        let mut network = test_data::create_test_network();
        network.substations.clear();

        network.register(&mut world, &mut schedule);

        let mut substation_query = world.query::<&Substation>();
        let substations: Vec<&Substation> = substation_query.iter(&world).collect();

        assert_eq!(substations.len(), 0);
    }

    #[test]
    fn test_network_register_idempotency() {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        world.init_resource::<AssetRegistry>();
        world.init_resource::<Events<RegisterEvent<Network>>>();
        world.init_resource::<Events<RegisterEvent<Substation>>>();
        schedule.add_systems(handle_register_events::<Network>);
        schedule.add_systems(handle_register_events::<Substation>);

        let network = test_data::create_test_network();

        // Register twice
        network.register(&mut world, &mut schedule);
        network.register(&mut world, &mut schedule);

        // Verify no duplicate registrations
        let mut substation_query = world.query::<&Substation>();
        let substations: Vec<&Substation> = substation_query.iter(&world).collect();

        assert_eq!(substations.len(), 2);
        assert!(substations.iter().any(|s| s.id == "sub1"));
        assert!(substations.iter().any(|s| s.id == "sub2"));
    }
}
