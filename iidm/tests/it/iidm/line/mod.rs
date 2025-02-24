use iidm::Line;

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
