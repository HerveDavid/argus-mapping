use crate::entities::*;
use crate::extensions::*;
use crate::plugins::*;
use crate::AssetRegistry;

use bevy_ecs::prelude::*;

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
        fn init_updatable_components(world: &mut World, schedule: &mut Schedule) {
             $(
                // Static verification that the type implements Updatable
                assert_updatable::<$component>();
                // Static verification that the type is a valid component
                assert_component::<$component>();

                world.init_resource::<Events<UpdateEvent<$component>>>();
                schedule.add_systems(handle_update_events::<$component>);
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

pub fn insert_iidm_ecs(mut world: &mut World, mut schedule: &mut Schedule) {
    // Init registries
    world.init_resource::<AssetRegistry>();

    // Init all iidm entities
    init_identifiable_component(&mut world, &mut schedule);
    init_updatable_components(&mut world, &mut schedule);

    // Init events
    world.insert_resource(Events::<EntityNotFoundEvent>::default());
}
