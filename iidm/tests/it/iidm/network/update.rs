use super::*;
use chrono::DateTime;
use iidm::*;
use std::str::FromStr;

#[test]
fn test_update_basic_fields() {
    let mut network = create_default_network();
    network.update(NetworkUpdater {
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

    network.update(NetworkUpdater::default());

    assert_eq!(
        serde_json::to_value(&network).unwrap(),
        serde_json::to_value(&original).unwrap()
    );
}
