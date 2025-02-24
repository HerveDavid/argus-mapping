mod identifiable;
mod serialization;
mod update;

use std::str::FromStr;

use chrono::DateTime;
use iidm::*;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";
const VALID_NETWORK_JSON: &str = r#"{
        "version": "1.12",
        "id": "sim1",
        "caseDate": "2013-01-15T18:45:00.000+01:00",
        "forecastDistance": 0,
        "sourceFormat": "test",
        "minimumValidationLevel": "STEADY_STATE_HYPOTHESIS",
        "substations": [],
        "lines": []
    }"#;

fn create_test_network() -> Network {
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
