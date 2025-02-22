use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::{
    handle_register_events, handle_update_events, AssetRegistry, Line, LineUpdate, RegisterEvent,
    UpdateEvent,
};

#[test]
fn test_handle_line_update() {
    // Init world
    let mut world = World::default();
    let mut schedule = Schedule::default();

    // Init Resources and Systems
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
    let line_update = LineUpdate {
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
        update: line_update,
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

    // Update line with event
    for update in updates {
        let expected = update.clone();

        // Update line with event
        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Line>>>();
        event_writer.send(UpdateEvent {
            id: "line1".to_string(),
            update,
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
