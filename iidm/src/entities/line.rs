use super::Line;

impl Line {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn update(&mut self, update_json: &str) -> Result<(), serde_json::Error> {
        let updates: serde_json::Value = serde_json::from_str(update_json)?;
        let mut current_json = serde_json::to_value(&self)?;

        if let (
            serde_json::Value::Object(ref mut current_map),
            serde_json::Value::Object(updates_map),
        ) = (&mut current_json, updates)
        {
            for (key, value) in updates_map {
                current_map.insert(key, value);
            }
            *self = serde_json::from_value(current_json)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::resources::{Id, PhysicalAssetRegistry};
    use serde_json::json;

    use super::*;
    use bevy_ecs::{
        system::Commands,
        world::{CommandQueue, World},
    };

    const LINE_STR: &str = r#"{
        "id" : "NHV1_NHV2_1",
        "r" : 3.0,
        "x" : 33.0,
        "g1" : 0.0,
        "b1" : 1.93E-4,
        "g2" : 0.0,
        "b2" : 1.93E-4,
        "voltageLevelId1" : "VLHV1",
        "bus1" : "NHV1",
        "connectableBus1" : "NHV1",
        "voltageLevelId2" : "VLHV2",
        "bus2" : "NHV2",
        "connectableBus2" : "NHV2"   
    }"#;

    #[test]
    fn test_line_load_from_str() -> Result<(), Box<dyn std::error::Error>> {
        let line = Line::from_json_str(&LINE_STR)?;

        assert_eq!(line.id, "NHV1_NHV2_1");

        assert_eq!(line.r, 3.0);
        assert_eq!(line.x, 33.0);
        assert_eq!(line.g1, 0.0);
        assert_eq!(line.b1, 1.93E-4);
        assert_eq!(line.g2, 0.0);
        assert_eq!(line.b2, 1.93E-4);

        assert_eq!(line.voltage_level_id1, "VLHV1");
        assert_eq!(line.bus1, "NHV1");
        assert_eq!(line.connectable_bus1, "NHV1");
        assert_eq!(line.voltage_level_id2, "VLHV2");
        assert_eq!(line.bus2, "NHV2");
        assert_eq!(line.connectable_bus2, "NHV2");

        Ok(())
    }

    #[test]
    fn test_spawn_component() -> Result<(), Box<dyn std::error::Error>> {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();
        let line = Line::from_json_str(&LINE_STR)?;

        let test_id = line.id.clone();
        {
            let mut commands = Commands::new(&mut queue, &world);
            let entity = registry.spawn_component(&mut commands, line);

            queue.apply(&mut world);

            let identifiable = world.get::<Id>(entity);
            assert!(identifiable.is_some());
            assert_eq!(identifiable.unwrap().0, test_id);

            assert!(registry.0.contains_key(&test_id));
            assert_eq!(registry.0.get(&test_id), Some(&entity));

            let mut line_query = world.query::<&Line>();
            for line_component in line_query.iter(&world) {
                assert_eq!(line_component.r, 3.0);
                assert_eq!(line_component.x, 33.0);
                assert_eq!(line_component.g1, 0.0);
                assert_eq!(line_component.b1, 1.93E-4);
                assert_eq!(line_component.g2, 0.0);
                assert_eq!(line_component.b2, 1.93E-4);

                assert_eq!(line_component.voltage_level_id1, "VLHV1");
                assert_eq!(line_component.bus1, "NHV1");
                assert_eq!(line_component.connectable_bus1, "NHV1");
                assert_eq!(line_component.voltage_level_id2, "VLHV2");
                assert_eq!(line_component.bus2, "NHV2");
                assert_eq!(line_component.connectable_bus2, "NHV2");
            }
        }

        Ok(())
    }

    #[test]
    fn test_mutable_line() -> Result<(), Box<dyn std::error::Error>> {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();
        let line = Line::from_json_str(&LINE_STR)?;

        {
            let mut commands = Commands::new(&mut queue, &world);
            let _ = registry.spawn_component(&mut commands, line);

            queue.apply(&mut world);

            // Modification des valeurs via une requête mutable
            let mut line_query = world.query::<&mut Line>();
            for mut line_component in line_query.iter_mut(&mut world) {
                // Modification des paramètres électriques
                line_component.r = 4.0;
                line_component.x = 44.0;
                line_component.g1 = 0.1;
                line_component.b1 = 2.93E-4;
                line_component.g2 = 0.1;
                line_component.b2 = 2.93E-4;

                // Modification des bus
                line_component.bus1 = "NHV1_NEW".to_string();
                line_component.bus2 = "NHV2_NEW".to_string();
            }

            // Vérification des nouvelles valeurs
            let mut check_query = world.query::<&Line>();
            for line_component in check_query.iter(&world) {
                assert_eq!(line_component.r, 4.0);
                assert_eq!(line_component.x, 44.0);
                assert_eq!(line_component.g1, 0.1);
                assert_eq!(line_component.b1, 2.93E-4);
                assert_eq!(line_component.g2, 0.1);
                assert_eq!(line_component.b2, 2.93E-4);

                assert_eq!(line_component.bus1, "NHV1_NEW");
                assert_eq!(line_component.bus2, "NHV2_NEW");

                // Vérification que les autres champs n'ont pas changé
                assert_eq!(line_component.voltage_level_id1, "VLHV1");
                assert_eq!(line_component.connectable_bus1, "NHV1");
                assert_eq!(line_component.voltage_level_id2, "VLHV2");
                assert_eq!(line_component.connectable_bus2, "NHV2");
            }
        }

        Ok(())
    }

    fn create_default_line() -> Line {
        Line {
            id: "line1".to_string(),
            r: 1.0,
            x: 2.0,
            g1: 3.0,
            b1: 4.0,
            g2: 5.0,
            b2: 6.0,
            voltage_level_id1: "vl1".to_string(),
            bus1: "bus1".to_string(),
            connectable_bus1: "cbus1".to_string(),
            voltage_level_id2: "vl2".to_string(),
            bus2: "bus2".to_string(),
            connectable_bus2: "cbus2".to_string(),
            current_limits1: None,
            current_limits2: None,
        }
    }

    #[test]
    fn test_update_single_field() -> Result<(), serde_json::Error> {
        let mut line = create_default_line();
        let update = json!({
            "r": 10.0
        })
        .to_string();

        line.update(&update)?;
        assert_eq!(line.r, 10.0);
        assert_eq!(line.x, 2.0);
        assert_eq!(line.id, "line1");
        Ok(())
    }

    #[test]
    fn test_update_multiple_fields() -> Result<(), serde_json::Error> {
        let mut line = create_default_line();
        let update = json!({
            "r": 10.0,
            "x": 20.0,
            "id": "newline"
        })
        .to_string();

        line.update(&update)?;
        assert_eq!(line.r, 10.0);
        assert_eq!(line.x, 20.0);
        assert_eq!(line.id, "newline");
        Ok(())
    }

    #[test]
    fn test_update_with_renamed_fields() -> Result<(), serde_json::Error> {
        let mut line = create_default_line();
        let update = json!({
            "voltageLevelId1": "newvl1",
            "connectableBus1": "newcbus1"
        })
        .to_string();

        line.update(&update)?;
        assert_eq!(line.voltage_level_id1, "newvl1");
        assert_eq!(line.connectable_bus1, "newcbus1");
        Ok(())
    }

    #[test]
    fn test_update_optional_fields() -> Result<(), serde_json::Error> {
        let mut line = create_default_line();
        let current_limits = json!({
            "permanentLimit": 100.0,
            "temporaryLimits": []
        });

        let update = json!({
            "currentLimits1": current_limits,
            "currentLimits2": null
        })
        .to_string();

        line.update(&update)?;
        assert!(line.current_limits1.is_some());
        assert!(line.current_limits2.is_none());
        Ok(())
    }

    #[test]
    fn test_update_with_invalid_json() {
        let mut line = create_default_line();
        let result = line.update("invalid json");
        assert!(result.is_err());
    }

    // #[test]
    // fn test_update_with_empty_json() -> Result<(), serde_json::Error> {
    //     let mut line = create_default_line();
    //     let original = line.clone();

    //     line.update("{}")?;
    //     assert_eq!(line.id, original.id);
    //     assert_eq!(line.r, original.r);
    //     assert_eq!(line.x, original.x);
    //     Ok(())
    // }
}
