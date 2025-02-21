use bevy_ecs::prelude::*;

use crate::Updatable;

#[derive(Event)]
pub struct UpdateEvent<T: Updatable>
where
    T: 'static,
    T::Event: Send + Sync,
{
    pub entity: Entity,
    pub updates: T::Event,
}

pub fn handle_update_events<T: Component + Updatable>(
    mut update_events: EventReader<UpdateEvent<T>>,
    mut query: Query<&mut T>,
) where
    T: 'static,
    T::Event: Send + Sync + Clone,
{
    for UpdateEvent { entity, updates } in update_events.read() {
        if let Ok(mut component) = query.get_mut(*entity) {
            component.update(updates.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Line, LineUpdate};

    #[test]
    fn test_handle_line_update() {
        // Création du monde
        let mut world = World::new();
        let mut schedule = Schedule::default();

        // Initialisation du système d'événements
        world.init_resource::<Events<UpdateEvent<Line>>>();
        schedule.add_systems(handle_update_events::<Line>);

        // Création d'une Line
        let line = Line {
            id: "line1".to_string(),
            r: 1.0,
            x: 2.0,
            g1: 3.0,
            b1: 4.0,
            g2: 5.0,
            b2: 6.0,
            voltage_level_id1: "vl1".to_string(),
            bus1: "bus1".to_string(),
            connectable_bus1: "bus1".to_string(),
            voltage_level_id2: "vl2".to_string(),
            bus2: "bus2".to_string(),
            connectable_bus2: "bus2".to_string(),
            current_limits1: None,
            current_limits2: None,
        };

        let entity = world.spawn(line).id();

        // Création et envoi d'un événement d'update
        let update_event = UpdateEvent {
            entity,
            updates: LineUpdate {
                r: Some(10.0),
                x: Some(20.0),
                g1: Some(30.0),
                b1: Some(40.0),
                ..Default::default()
            },
        };

        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Line>>>();
        event_writer.send(update_event);

        // Exécution du schedule
        schedule.run(&mut world);

        // Vérification que l'update a bien été appliquée
        let line = world.entity(entity).get::<Line>().unwrap();
        assert_eq!(line.r, 10.0);
        assert_eq!(line.x, 20.0);
        assert_eq!(line.g1, 30.0);
        assert_eq!(line.b1, 40.0);
        // Vérifie que les autres champs n'ont pas été modifiés
        assert_eq!(line.g2, 5.0);
        assert_eq!(line.b2, 6.0);
    }

    #[test]
    fn test_multiple_line_updates() {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        world.init_resource::<Events<UpdateEvent<Line>>>();
        schedule.add_systems(handle_update_events::<Line>);

        let entity = world
            .spawn(Line {
                id: "line1".to_string(),
                r: 1.0,
                x: 2.0,
                g1: 3.0,
                b1: 4.0,
                g2: 5.0,
                b2: 6.0,
                voltage_level_id1: "vl1".to_string(),
                bus1: "bus1".to_string(),
                connectable_bus1: "bus1".to_string(),
                voltage_level_id2: "vl2".to_string(),
                bus2: "bus2".to_string(),
                connectable_bus2: "bus2".to_string(),
                current_limits1: None,
                current_limits2: None,
            })
            .id();

        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Line>>>();

        let updates = vec![
            LineUpdate {
                r: Some(10.0),
                x: Some(20.0),
                g1: Some(30.0),
                b1: Some(40.0),
                ..Default::default()
            },
            LineUpdate {
                r: Some(11.0),
                x: Some(21.0),
                g1: Some(31.0),
                b1: Some(41.0),
                ..Default::default()
            },
            LineUpdate {
                r: Some(12.0),
                x: Some(22.0),
                g1: Some(32.0),
                b1: Some(42.0),
                ..Default::default()
            },
        ];

        for update in updates {
            event_writer.send(UpdateEvent {
                entity,
                updates: update,
            });
        }

        schedule.run(&mut world);

        let line = world.entity(entity).get::<Line>().unwrap();
        assert_eq!(line.r, 12.0);
        assert_eq!(line.x, 22.0);
        assert_eq!(line.g1, 32.0);
        assert_eq!(line.b1, 42.0);
    }
}
