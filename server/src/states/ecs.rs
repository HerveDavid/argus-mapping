mod update_registry;

use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::*;
use tokio::sync::RwLock;
use update_registry::UpdateRegistry;

trait ComponentInit: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> ComponentInit for T {}

fn assert_identifiable<T: Identifiable>() {}
fn assert_component<T: ComponentInit>() {}
fn assert_updatable<T: Updatable>() {}

macro_rules! init_identifiable_components {
    ($($component:ty),* $(,)?) => {
        fn init_identifiable_component(world: &mut World, schedule: &mut Schedule) {
            $(
                // Static verification that the type implements Identifiable
                assert_identifiable::<$component>();

                // Static verification that the type is a valid component
                assert_component::<$component>();

                world.init_resource::<Events<RegisterEvent<$component>>>();
                schedule.add_systems(handle_register_events::<$component>);
            )*
        }
    };
}

macro_rules! init_updatable_components {
    ($($component:ty),* $(,)?) => {
        fn init_updatable_components(world: &mut World, schedule: &mut Schedule, updater: &mut UpdateRegistry) {
             $(
                // Static verification that the type implements Updatable
                assert_updatable::<$component>();
                // Static verification that the type is a valid component
                assert_component::<$component>();
                world.init_resource::<Events<UpdateEvent<$component>>>();
                schedule.add_systems(handle_update_events::<$component>);

                // Register component type with update registry
                // Converting PascalCase to snake_case for component name
                let component_name = stringify!($component)
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, c)| {
                        if i > 0 && c.is_uppercase() {
                            acc.push('_');
                            acc.push(c.to_lowercase().next().unwrap());
                        } else {
                            acc.push(c.to_lowercase().next().unwrap());
                        }
                        acc
                    });

                // Register component type with its corresponding updater and error types
                updater.register::<$component, paste::paste! {[<$component Updater>]}, paste::paste! {[<$component Error>]}>(&component_name);
            )*
        }
    };
}

init_identifiable_components!(
    Network,
    Line,
    Substation,
    VoltageLevel,
    Generator,
    Load,
    Bus,
    BusbarSection,
    TwoWindingsTransformer,
    ThreeWindingsTransformer,
    Switch,
    ShuntCompensator,
    StaticVarCompensator,
    DanglingLine,
    TieLine,
    HvdcLine,
    HvdcConverterStation,
    TerminalRef
);

init_updatable_components!(
    Network,
    Line,
    Substation,
    VoltageLevel,
    Generator,
    Load,
    Bus,
    BusbarSection,
    TwoWindingsTransformer,
    ThreeWindingsTransformer,
    Switch,
    ShuntCompensator,
    StaticVarCompensator,
    DanglingLine,
    TieLine,
    HvdcLine,
    HvdcConverterStation,
    ReactiveCapabilityCurve,
    ReactiveCapabilityCurvePoint,
    MinMaxReactiveLimits,
    ExponentialLoadModel,
    ZipLoadModel,
    BusBreakerTopology,
    NodeBreakerTopology,
    Node,
    InternalConnection,
    RatioTapChanger,
    PhaseTapChanger,
    TapStep,
    PhaseTapStep,
    CurrentLimits,
    TemporaryLimit
);

pub struct EcsState {
    pub world: RwLock<World>,
    pub schedule: RwLock<Schedule>,
    pub update_registry: RwLock<UpdateRegistry>,
}

impl Default for EcsState {
    fn default() -> Self {
        // Init world
        let mut world = World::default();
        let mut schedule = Schedule::default();

        // Init registry
        let mut update_registry = UpdateRegistry::default();
        world.init_resource::<AssetRegistry>();

        // Init Resources and Systems
        init_identifiable_component(&mut world, &mut schedule);
        init_updatable_components(&mut world, &mut schedule, &mut update_registry);

        // Init Errors handler
        world.insert_resource(Events::<EntityNotFoundEvent>::default());

        Self {
            world: RwLock::new(world),
            schedule: RwLock::new(schedule),
            update_registry: RwLock::new(update_registry),
        }
    }
}
