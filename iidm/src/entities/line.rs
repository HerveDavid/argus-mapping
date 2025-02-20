use super::Line;

impl Line {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_data {
        use super::*;

        pub const VALID_LINE_JSON: &str = r#"{
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

        pub fn create_default_line() -> Line {
            Line::from_json_str(VALID_LINE_JSON).unwrap()
        }

        pub fn assert_default_values(line: &Line) {
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
    }

    mod serialization {
        use super::*;
        use test_data::*;

        #[test]
        fn test_deserialize_from_json() {
            let line = Line::from_json_str(VALID_LINE_JSON).unwrap();
            assert_default_values(&line);
        }

        #[test]
        fn test_serialize_to_json() {
            let line = create_default_line();
            let json = line.to_json_string().unwrap();
            let deserialized: Line = serde_json::from_str(&json).unwrap();
            assert_default_values(&deserialized);
        }
    }

    mod updates {
        use super::*;
        use crate::CurrentLimits;
        use crate::LineUpdate;
        use crate::TemporaryLimit;
        use test_data::*;

        #[test]
        fn test_update_single_field() {
            let mut line = create_default_line();
            line.update(LineUpdate {
                r: Some(10.0),
                ..Default::default()
            });
            assert_eq!(line.r, 10.0);
            assert_eq!(line.x, 33.0);
            assert_eq!(line.g1, 0.0);
            assert_eq!(line.b1, 1.93E-4);
        }

        #[test]
        fn test_update_multiple_fields() {
            let mut line = create_default_line();
            line.update(LineUpdate {
                r: Some(10.0),
                x: Some(20.0),
                g1: Some(1.0),
                b1: Some(2.0),
                ..Default::default()
            });
            assert_eq!(line.r, 10.0);
            assert_eq!(line.x, 20.0);
            assert_eq!(line.g1, 1.0);
            assert_eq!(line.b1, 2.0);
            assert_eq!(line.g2, 0.0);
            assert_eq!(line.b2, 1.93E-4);
        }

        #[test]
        fn test_update_connection_fields() {
            let mut line = create_default_line();
            line.update(LineUpdate {
                voltage_level_id1: Some("NEW_VL1".to_string()),
                bus1: Some("NEW_BUS1".to_string()),
                connectable_bus1: Some("NEW_CBUS1".to_string()),
                ..Default::default()
            });
            assert_eq!(line.voltage_level_id1, "NEW_VL1");
            assert_eq!(line.bus1, "NEW_BUS1");
            assert_eq!(line.connectable_bus1, "NEW_CBUS1");
            assert_eq!(line.voltage_level_id2, "VLHV2");
            assert_eq!(line.bus2, "NHV2");
        }

        #[test]
        fn test_update_current_limits() {
            let mut line = create_default_line();
            let new_limits = CurrentLimits {
                permanent_limit: 1000.0,
                temporary_limits: vec![TemporaryLimit {
                    name: "limit1".to_string(),
                    acceptable_duration: 20,
                    value: 1200.0,
                }],
            };

            line.update(LineUpdate {
                current_limits1: Some(Some(new_limits)),
                ..Default::default()
            });

            assert!(line.current_limits1.is_some());
            assert!(line.current_limits2.is_none());

            let limits1 = line.current_limits1.as_ref().unwrap();
            assert_eq!(limits1.permanent_limit, 1000.0);
            assert_eq!(limits1.temporary_limits.len(), 1);
            assert_eq!(limits1.temporary_limits[0].name, "limit1");
        }

        #[test]
        fn test_update_remove_current_limits() {
            let mut line = create_default_line();
            // Add
            line.update(LineUpdate {
                current_limits1: Some(Some(CurrentLimits {
                    permanent_limit: 1000.0,
                    temporary_limits: vec![],
                })),
                ..Default::default()
            });

            // And remove
            line.update(LineUpdate {
                current_limits1: Some(None),
                ..Default::default()
            });

            assert!(line.current_limits1.is_none());
        }

        #[test]
        fn test_update_with_empty_update() {
            let mut line = create_default_line();
            let original = create_default_line();

            line.update(LineUpdate::default());

            assert_eq!(
                serde_json::to_value(&line).unwrap(),
                serde_json::to_value(&original).unwrap()
            );
        }
    }

    mod json_updates {
        use super::*;
        use test_data::*;

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
    }
}
