use super::*;
use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::*;

#[test]
fn test_update_single_field() {
    let mut line = create_default_line();
    line.update(LineUpdater {
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
    line.update(LineUpdater {
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
    line.update(LineUpdater {
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

    line.update(LineUpdater {
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
    line.update(LineUpdater {
        current_limits1: Some(Some(CurrentLimits {
            permanent_limit: 1000.0,
            temporary_limits: vec![],
        })),
        ..Default::default()
    });

    // And remove
    line.update(LineUpdater {
        current_limits1: Some(None),
        ..Default::default()
    });

    assert!(line.current_limits1.is_none());
}

#[test]
fn test_update_with_empty_update() {
    let mut line = create_default_line();
    let original = create_default_line();

    line.update(LineUpdater::default());

    assert_eq!(
        serde_json::to_value(&line).unwrap(),
        serde_json::to_value(&original).unwrap()
    );
}

#[test]
fn test_handle_line_update() {
    // Init world
    let mut world = World::default();
    let mut schedule = Schedule::default();

    // Init Resources and Systems
    world.init_resource::<Events<EntityNotFoundEvent>>();
    world.init_resource::<Events<RegisterEvent<Line>>>();
    world.init_resource::<Events<UpdateEvent<Line>>>();
    world.init_resource::<AssetRegistry>();
    schedule.add_systems(handle_register_events::<Line>);
    schedule.add_systems(handle_update_events::<Line>);

    // Add line
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

    // Register line with event
    let mut event_writer = world.resource_mut::<Events<RegisterEvent<Line>>>();
    event_writer.send(RegisterEvent {
        id: "line1".to_string(),
        component: line,
    });

    // Run the schedule world to apply change
    schedule.run(&mut world);

    // Check current state
    let registry = world.resource::<AssetRegistry>();
    let entity = registry.find("line1").unwrap();
    let line = world.entity(entity).get::<Line>().unwrap();
    assert_eq!(line.r, 1.0);
    assert_eq!(line.x, 2.0);
    assert_eq!(line.g1, 3.0);
    assert_eq!(line.b1, 4.0);

    // Init a update
    let line_update = LineUpdater {
        r: Some(10.0),
        x: Some(20.0),
        g1: Some(30.0),
        b1: Some(40.0),
        ..Default::default()
    };

    // Update line with event
    let mut event_writer = world.resource_mut::<Events<UpdateEvent<Line>>>();
    event_writer.send(UpdateEvent {
        id: "line1".to_string(),
        updater: line_update,
    });

    // Run the schedule world to apply change
    schedule.run(&mut world);

    // Check changed state
    let registry = world.resource::<AssetRegistry>();
    let entity = registry.find("line1").unwrap();
    let line = world.entity(entity).get::<Line>().unwrap();
    assert_eq!(line.r, 10.0);
    assert_eq!(line.x, 20.0);
    assert_eq!(line.g1, 30.0);
    assert_eq!(line.b1, 40.0);
}

#[test]
fn test_multiple_line_updates() {
    // Init world
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Init Resources and Systems
    world.init_resource::<Events<EntityNotFoundEvent>>();
    world.init_resource::<Events<RegisterEvent<Line>>>();
    world.init_resource::<Events<UpdateEvent<Line>>>();
    world.init_resource::<AssetRegistry>();
    schedule.add_systems(handle_register_events::<Line>);
    schedule.add_systems(handle_update_events::<Line>);

    // Add line
    let component = Line {
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

    // Register line with event
    let mut event_writer = world.resource_mut::<Events<RegisterEvent<Line>>>();
    event_writer.send(RegisterEvent {
        id: "line1".to_string(),
        component,
    });

    // Run the schedule world to apply change
    schedule.run(&mut world);

    // List of updates
    let updates = vec![
        LineUpdater {
            r: Some(10.0),
            x: Some(20.0),
            g1: Some(30.0),
            b1: Some(40.0),
            ..Default::default()
        },
        LineUpdater {
            r: Some(11.0),
            x: Some(21.0),
            g1: Some(31.0),
            b1: Some(41.0),
            ..Default::default()
        },
        LineUpdater {
            r: Some(12.0),
            x: Some(22.0),
            g1: Some(32.0),
            b1: Some(42.0),
            ..Default::default()
        },
    ];

    // Update line with event
    for update in updates {
        let expected = update.clone();

        // Update line with event
        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Line>>>();
        event_writer.send(UpdateEvent {
            id: "line1".to_string(),
            updater: update,
        });
        schedule.run(&mut world);

        // Check changed state
        let registry = world.resource::<AssetRegistry>();
        let entity = registry.find("line1").unwrap();
        let line = world.entity(entity).get::<Line>().unwrap();
        assert_eq!(line.r, expected.r.unwrap());
        assert_eq!(line.x, expected.x.unwrap());
        assert_eq!(line.g1, expected.g1.unwrap());
        assert_eq!(line.b1, expected.b1.unwrap());
    }
}
