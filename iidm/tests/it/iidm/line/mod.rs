use iidm::Line;

mod serialization;
mod update;

const VALID_LINE_JSON: &str = r#"{
            "id": "NHV1_NHV2_1",
            "r": 3.0,
            "x": 33.0,
            "g1": 0.0,
            "b1": 1.93E-4,
            "g2": 0.0,
            "b2": 1.93E-4,
            "voltageLevelId1": "VLHV1",
            "bus1": "NHV1",
            "connectableBus1": "NHV1",
            "voltageLevelId2": "VLHV2",
            "bus2": "NHV2",
            "connectableBus2": "NHV2"   
        }"#;

fn create_default_line() -> Line {
    serde_json::from_str(VALID_LINE_JSON).unwrap()
}

fn assert_default_values(line: &Line) {
    // Electrical values
    assert_eq!(line.id, "NHV1_NHV2_1");
    assert_eq!(line.r, 3.0);
    assert_eq!(line.x, 33.0);
    assert_eq!(line.g1, 0.0);
    assert_eq!(line.b1, 1.93E-4);
    assert_eq!(line.g2, 0.0);
    assert_eq!(line.b2, 1.93E-4);

    // Connectable data 1
    assert_eq!(line.voltage_level_id1, "VLHV1");
    assert_eq!(line.bus1, "NHV1");
    assert_eq!(line.connectable_bus1, "NHV1");

    // Connectable data 2
    assert_eq!(line.voltage_level_id2, "VLHV2");
    assert_eq!(line.bus2, "NHV2");
    assert_eq!(line.connectable_bus2, "NHV2");

    // Optional current limits
    assert!(line.current_limits1.is_none());
    assert!(line.current_limits2.is_none());
}
