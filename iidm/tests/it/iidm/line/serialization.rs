use iidm::{Line, Updatable};

use super::{assert_default_values, create_default_line, VALID_LINE_JSON};

#[test]
fn test_deserialize_from_json() {
    let line: Line = serde_json::from_str(VALID_LINE_JSON).unwrap();
    assert_default_values(&line);
}

#[test]
fn test_serialize_to_json() {
    let line = create_default_line();
    let json = serde_json::to_string(&line).unwrap();
    let deserialized: Line = serde_json::from_str(&json).unwrap();
    assert_default_values(&deserialized);
}

#[test]
fn test_json_update_with_invalid_json() {
    let mut line = create_default_line();
    assert!(line.update_from_json("invalid json").is_err());
}

#[test]
fn test_json_update_with_empty_json() {
    let mut line = create_default_line();
    let original = create_default_line();
    line.update_from_json("{}").unwrap();
    assert_eq!(
        serde_json::to_value(&line).unwrap(),
        serde_json::to_value(&original).unwrap()
    );
}

#[test]
fn test_json_update_renamed_fields() {
    let mut line = create_default_line();
    line.update_from_json(r#"{"voltageLevelId1": "newvl1"}"#)
        .unwrap();
    assert_eq!(line.voltage_level_id1, "newvl1");
}
