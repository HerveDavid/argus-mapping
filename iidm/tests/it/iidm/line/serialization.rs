use iidm::Line;

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
