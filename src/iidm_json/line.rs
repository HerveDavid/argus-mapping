use crate::iidm_ecs::{IdentifiableExt, PhysicalAssetRegistry};

use super::Line;
use bevy_ecs::prelude::*;

impl Line {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn load_in_registery(
        &self,
        commands: &mut Commands,
        registery: &mut PhysicalAssetRegistry,
    ) {
        registery.spawn_physical_asset(commands, self.id.clone());
    }
}

impl IdentifiableExt for Line {
    fn id(&self) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::iidm_ecs::Identifiable;

    use super::*;
    use bevy_ecs::world::CommandQueue;

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

        // Test de l'identifiant
        assert_eq!(line.id, "NHV1_NHV2_1");

        // Test des propriétés numériques
        assert_eq!(line.r, 3.0);
        assert_eq!(line.x, 33.0);
        assert_eq!(line.g1, 0.0);
        assert_eq!(line.b1, 1.93E-4);
        assert_eq!(line.g2, 0.0);
        assert_eq!(line.b2, 1.93E-4);

        // Test des propriétés de connexion
        assert_eq!(line.voltage_level_id1, "VLHV1");
        assert_eq!(line.bus1, "NHV1");
        assert_eq!(line.connectable_bus1, "NHV1");
        assert_eq!(line.voltage_level_id2, "VLHV2");
        assert_eq!(line.bus2, "NHV2");
        assert_eq!(line.connectable_bus2, "NHV2");

        Ok(())
    }

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut registry = PhysicalAssetRegistry::default();
        let line = Line::from_json_str(&LINE_STR)?;

        let test_id = line.id.clone();
        {
            let mut commands = Commands::new(&mut queue, &world);
            let entity = registry.spawn_component(&mut commands, line);

            queue.apply(&mut world);

            let identifiable = world.get::<Identifiable>(entity);
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
}
