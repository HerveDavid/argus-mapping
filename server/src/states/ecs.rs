use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::{
    handle_register_events, handle_update_events, AssetRegistry, Network, RegisterEvent,
    UpdateEvent,
};
use tokio::sync::RwLock;

pub struct EcsState {
    pub world: RwLock<World>,
    pub schedule: RwLock<Schedule>,
}

impl Default for EcsState {
    fn default() -> Self {
        // Init world
        let mut world = World::default();
        let mut schedule = Schedule::default();

        // Init Resources and Systems
        world.init_resource::<Events<RegisterEvent<Network>>>();
        world.init_resource::<Events<UpdateEvent<Network>>>();
        world.init_resource::<AssetRegistry>();
        schedule.add_systems(handle_register_events::<Network>);
        schedule.add_systems(handle_update_events::<Network>);

        Self {
            world: RwLock::new(world),
            schedule: RwLock::new(schedule),
        }
    }
}
