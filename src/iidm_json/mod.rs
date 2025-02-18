use bevy_ecs::component::Component;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub version: String,
    pub id: String,
    #[serde(rename = "caseDate")]
    pub case_date: DateTime<FixedOffset>,
    #[serde(rename = "forecastDistance")]
    pub forecast_distance: i32,
    #[serde(rename = "sourceFormat")]
    pub source_format: String,
    #[serde(rename = "minimumValidationLevel")]
    pub minimum_validation_level: String,
    pub substations: Vec<Substation>,
    pub lines: Vec<Line>,
    #[serde(rename = "threeWindingsTransformers")]
    pub three_windings_transformers: Vec<ThreeWindingsTransformer>,
    pub switches: Vec<Switch>,
    #[serde(rename = "shuntCompensators")]
    pub shunt_compensators: Vec<ShuntCompensator>,
    #[serde(rename = "staticVarCompensators")]
    pub static_var_compensators: Vec<StaticVarCompensator>,
    #[serde(rename = "danglingLines")]
    pub dangling_lines: Vec<DanglingLine>,
    #[serde(rename = "tieLines")]
    pub tie_lines: Vec<TieLine>,
    #[serde(rename = "hvdcLines")]
    pub hvdc_lines: Vec<HvdcLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Substation {
    pub id: String,
    pub country: String,
    pub tso: String,
    #[serde(rename = "geographicalTags")]
    pub geographical_tags: Vec<String>,
    #[serde(rename = "voltageLevels")]
    pub voltage_levels: Vec<VoltageLevel>,
    #[serde(rename = "twoWindingsTransformers")]
    pub two_windings_transformers: Vec<TwoWindingsTransformer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoltageLevel {
    pub id: String,
    #[serde(rename = "nominalV")]
    pub nominal_v: f64,
    #[serde(rename = "topologyKind")]
    pub topology_kind: TopologyKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<Generator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loads: Option<Vec<Load>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busbar_sections: Option<Vec<BusbarSection>>,
    #[serde(rename = "nodeBreakerTopology")]
    pub node_breaker_topology: Option<NodeBreakerTopology>,
    #[serde(rename = "busBreakerTopology")]
    pub bus_breaker_topology: Option<BusBreakerTopology>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TopologyKind {
    NodeBreaker,
    BusBreaker,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnergySource {
    Hydro,
    Nuclear,
    Wind,
    Thermal,
    Solar,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    pub id: String,
    #[serde(rename = "energySource")]
    pub energy_source: EnergySource,
    #[serde(rename = "minP")]
    pub min_p: f64,
    #[serde(rename = "maxP")]
    pub max_p: f64,
    #[serde(rename = "voltageRegulatorOn")]
    pub voltage_regulator_on: bool,
    #[serde(rename = "targetP")]
    pub target_p: f64,
    #[serde(rename = "targetV")]
    pub target_v: f64,
    #[serde(rename = "targetQ")]
    pub target_q: f64,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
    #[serde(rename = "reactiveCapabilityCurve")]
    pub reactive_capability_curve: Option<ReactiveCapabilityCurve>,
    #[serde(rename = "minMaxReactiveLimits")]
    pub min_max_reactive_limits: Option<MinMaxReactiveLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactiveCapabilityCurve {
    pub points: Vec<ReactiveCapabilityCurvePoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactiveCapabilityCurvePoint {
    pub p: f64,
    #[serde(rename = "minQ")]
    pub min_q: f64,
    #[serde(rename = "maxQ")]
    pub max_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinMaxReactiveLimits {
    #[serde(rename = "minQ")]
    pub min_q: f64,
    #[serde(rename = "maxQ")]
    pub max_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    pub id: String,
    #[serde(rename = "loadType")]
    pub load_type: LoadType,
    pub p0: f64,
    pub q0: f64,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
    #[serde(rename = "exponentialModel")]
    pub exponential_model: Option<ExponentialLoadModel>,
    #[serde(rename = "zipModel")]
    pub zip_model: Option<ZipLoadModel>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    Undefined,
    Auxiliary,
    Fictitious,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExponentialLoadModel {
    pub p0: f64,
    pub q0: f64,
    pub np: f64,
    pub nq: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZipLoadModel {
    pub p0: f64,
    pub q0: f64,
    #[serde(rename = "zP")]
    pub z_p: f64,
    #[serde(rename = "zQ")]
    pub z_q: f64,
    #[serde(rename = "iP")]
    pub i_p: f64,
    #[serde(rename = "iQ")]
    pub i_q: f64,
    #[serde(rename = "pP")]
    pub p_p: f64,
    #[serde(rename = "pQ")]
    pub p_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusBreakerTopology {
    pub buses: Vec<Bus>,
    pub switches: Vec<Switch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeBreakerTopology {
    pub nodes: Vec<Node>,
    pub switches: Vec<Switch>,
    #[serde(rename = "internalConnections")]
    pub internal_connections: Vec<InternalConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalConnection {
    pub node1: i32,
    pub node2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bus {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusbarSection {
    pub id: String,
    pub name: String,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoWindingsTransformer {
    pub id: String,
    pub r: f64,
    pub x: f64,
    pub g: f64,
    pub b: f64,
    #[serde(rename = "ratedU1")]
    pub rated_u1: f64,
    #[serde(rename = "ratedU2")]
    pub rated_u2: f64,
    #[serde(rename = "voltageLevelId1")]
    pub voltage_level_id1: String,
    pub bus1: String,
    #[serde(rename = "connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "voltageLevelId2")]
    pub voltage_level_id2: String,
    pub bus2: String,
    #[serde(rename = "connectableBus2")]
    pub connectable_bus2: String,
    #[serde(rename = "ratioTapChanger")]
    pub ratio_tap_changer: Option<RatioTapChanger>,
    #[serde(rename = "phaseTapChanger")]
    pub phase_tap_changer: Option<PhaseTapChanger>,
    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,
    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeWindingsTransformer {
    pub id: String,
    pub r1: f64,
    pub x1: f64,
    pub g1: f64,
    pub b1: f64,
    #[serde(rename = "ratedU1")]
    pub rated_u1: f64,
    pub r2: f64,
    pub x2: f64,
    pub g2: f64,
    pub b2: f64,
    #[serde(rename = "ratedU2")]
    pub rated_u2: f64,
    pub r3: f64,
    pub x3: f64,
    pub g3: f64,
    pub b3: f64,
    #[serde(rename = "ratedU3")]
    pub rated_u3: f64,
    #[serde(rename = "voltageLevelId1")]
    pub voltage_level_id1: String,
    #[serde(rename = "voltageLevelId2")]
    pub voltage_level_id2: String,
    #[serde(rename = "voltageLevelId3")]
    pub voltage_level_id3: String,
    pub bus1: String,
    pub bus2: String,
    pub bus3: String,
    #[serde(rename = "connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "connectableBus2")]
    pub connectable_bus2: String,
    #[serde(rename = "connectableBus3")]
    pub connectable_bus3: String,
    #[serde(rename = "ratioTapChanger2")]
    pub ratio_tap_changer2: Option<RatioTapChanger>,
    #[serde(rename = "ratioTapChanger3")]
    pub ratio_tap_changer3: Option<RatioTapChanger>,
    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,
    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
    #[serde(rename = "currentLimits3")]
    pub current_limits3: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatioTapChanger {
    pub regulating: bool,
    #[serde(rename = "lowTapPosition")]
    pub low_tap_position: i32,
    #[serde(rename = "tapPosition")]
    pub tap_position: i32,
    #[serde(rename = "targetDeadband")]
    pub target_deadband: f64,
    #[serde(rename = "loadTapChangingCapabilities")]
    pub load_tap_changing_capabilities: bool,
    #[serde(rename = "regulationMode")]
    pub regulation_mode: RatioRegulationMode,
    #[serde(rename = "regulationValue")]
    pub regulation_value: f64,
    #[serde(rename = "terminalRef")]
    pub terminal_ref: TerminalRef,
    pub steps: Vec<TapStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseTapChanger {
    pub regulating: bool,
    #[serde(rename = "lowTapPosition")]
    pub low_tap_position: i32,
    #[serde(rename = "tapPosition")]
    pub tap_position: i32,
    #[serde(rename = "regulationMode")]
    pub regulation_mode: PhaseRegulationMode,
    #[serde(rename = "regulationValue")]
    pub regulation_value: f64,
    #[serde(rename = "targetDeadband")]
    pub target_deadband: f64,
    #[serde(rename = "terminalRef")]
    pub terminal_ref: TerminalRef,
    pub steps: Vec<PhaseTapStep>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhaseRegulationMode {
    CurrentLimiter,
    ActivePowerControl,
    FixedTap,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RatioRegulationMode {
    Voltage,
    ReactivePower,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: String,
    pub r: f64,
    pub x: f64,
    pub g1: f64,
    pub b1: f64,
    pub g2: f64,
    pub b2: f64,
    #[serde(rename = "voltageLevelId1")]
    pub voltage_level_id1: String,
    pub bus1: String,
    #[serde(rename = "connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "voltageLevelId2")]
    pub voltage_level_id2: String,
    pub bus2: String,
    #[serde(rename = "connectableBus2")]
    pub connectable_bus2: String,
    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,
    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Switch {
    pub id: String,
    pub kind: SwitchKind,
    pub open: bool,
    pub retained: bool,
    pub bus1: String,
    pub bus2: String,
    #[serde(rename = "voltageLevelId")]
    pub voltage_level_id: String,
}

#[derive(Debug, Serialize, Deserialize, Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SwitchKind {
    Breaker,
    Disconnector,
    LoadBreakSwitch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShuntCompensator {
    pub id: String,
    #[serde(rename = "bPerSection")]
    pub b_per_section: f64,
    #[serde(rename = "maximumSectionCount")]
    pub maximum_section_count: i32,
    #[serde(rename = "sectionCount")]
    pub section_count: i32,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticVarCompensator {
    pub id: String,
    #[serde(rename = "bMin")]
    pub b_min: f64,
    #[serde(rename = "bMax")]
    pub b_max: f64,
    #[serde(rename = "regulationMode")]
    pub regulation_mode: StaticVarCompensatorRegulationMode,
    #[serde(rename = "voltageSetpoint")]
    pub voltage_setpoint: f64,
    #[serde(rename = "reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StaticVarCompensatorRegulationMode {
    Voltage,
    ReactivePower,
    Off,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DanglingLine {
    pub id: String,
    pub p0: f64,
    pub q0: f64,
    pub r: f64,
    pub x: f64,
    pub g: f64,
    pub b: f64,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TieLine {
    pub id: String,
    pub name: String,
    #[serde(rename = "danglingLine1")]
    pub dangling_line1: DanglingLine,
    #[serde(rename = "danglingLine2")]
    pub dangling_line2: DanglingLine,
    #[serde(rename = "ucteXnodeCode")]
    pub ucte_xnode_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HvdcLine {
    pub id: String,
    pub name: String,
    #[serde(rename = "r")]
    pub resistance: f64,
    #[serde(rename = "nominalV")]
    pub nominal_v: f64,
    #[serde(rename = "convertersMode")]
    pub converters_mode: ConvertersMode,
    #[serde(rename = "activePowerSetpoint")]
    pub active_power_setpoint: f64,
    #[serde(rename = "maxP")]
    pub max_p: f64,
    #[serde(rename = "converterStation1")]
    pub converter_station1: HvdcConverterStation,
    #[serde(rename = "converterStation2")]
    pub converter_station2: HvdcConverterStation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HvdcConverterStation {
    pub id: String,
    pub name: String,
    #[serde(rename = "voltageLevelId")]
    pub voltage_level_id: String,
    pub bus: String,
    #[serde(rename = "connectableBus")]
    pub connectable_bus: String,
    #[serde(rename = "lossFactor")]
    pub loss_factor: f64,
    #[serde(rename = "reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConvertersMode {
    Side1RectifierSide2Inverter,
    Side1InverterSide2Rectifier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalRef {
    pub id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TapStep {
    pub r: f64,
    pub x: f64,
    pub g: f64,
    pub b: f64,
    pub rho: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseTapStep {
    pub r: f64,
    pub x: f64,
    pub g: f64,
    pub b: f64,
    pub rho: f64,
    pub alpha: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentLimits {
    #[serde(rename = "permanentLimit")]
    pub permanent_limit: f64,
    #[serde(rename = "temporaryLimits")]
    pub temporary_limits: Vec<TemporaryLimit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryLimit {
    pub name: String,
    #[serde(rename = "acceptableDuration")]
    pub acceptable_duration: i32,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    One,
    Two,
    Three,
}
