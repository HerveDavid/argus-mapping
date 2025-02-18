use serde::{Deserialize, Serialize};

mod components;
mod ecs_bus_nodes;
mod ecs_edges;
mod ecs_nodes;

pub use components::*;
pub use ecs_bus_nodes::*;
pub use ecs_edges::*;
pub use ecs_nodes::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayoutParameters {
    #[serde(rename = "textNodesForceLayout")]
    pub text_nodes_force_layout: bool,
    #[serde(rename = "springRepulsionFactorForceLayout")]
    pub spring_repulsion_factor_force_layout: f64,
    #[serde(rename = "textNodeFixedShift")]
    pub text_node_fixed_shift: Point,
    #[serde(rename = "maxSteps")]
    pub max_steps: i32,
    #[serde(rename = "textNodeEdgeConnectionYShift")]
    pub text_node_edge_connection_y_shift: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Padding {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SvgParameters {
    #[serde(rename = "diagramPadding")]
    pub diagram_padding: Padding,
    #[serde(rename = "insertNameDesc")]
    pub insert_name_desc: bool,
    #[serde(rename = "svgWidthAndHeightAdded")]
    pub svg_width_and_height_added: bool,
    #[serde(rename = "cssLocation")]
    pub css_location: String,
    #[serde(rename = "sizeConstraint")]
    pub size_constraint: String,
    #[serde(rename = "fixedWidth")]
    pub fixed_width: f64,
    #[serde(rename = "fixedHeight")]
    pub fixed_height: f64,
    #[serde(rename = "fixedScale")]
    pub fixed_scale: f64,
    #[serde(rename = "arrowShift")]
    pub arrow_shift: f64,
    #[serde(rename = "arrowLabelShift")]
    pub arrow_label_shift: f64,
    #[serde(rename = "converterStationWidth")]
    pub converter_station_width: f64,
    #[serde(rename = "voltageLevelCircleRadius")]
    pub voltage_level_circle_radius: f64,
    #[serde(rename = "fictitiousVoltageLevelCircleRadius")]
    pub fictitious_voltage_level_circle_radius: f64,
    #[serde(rename = "transformerCircleRadius")]
    pub transformer_circle_radius: f64,
    #[serde(rename = "nodeHollowWidth")]
    pub node_hollow_width: f64,
    #[serde(rename = "edgesForkLength")]
    pub edges_fork_length: f64,
    #[serde(rename = "edgesForkAperture")]
    pub edges_fork_aperture: f64,
    #[serde(rename = "edgeStartShift")]
    pub edge_start_shift: f64,
    #[serde(rename = "unknownBusNodeExtraRadius")]
    pub unknown_bus_node_extra_radius: f64,
    #[serde(rename = "loopDistance")]
    pub loop_distance: f64,
    #[serde(rename = "loopEdgesAperture")]
    pub loop_edges_aperture: f64,
    #[serde(rename = "loopControlDistance")]
    pub loop_control_distance: f64,
    #[serde(rename = "edgeInfoAlongEdge")]
    pub edge_info_along_edge: bool,
    #[serde(rename = "edgeNameDisplayed")]
    pub edge_name_displayed: bool,
    #[serde(rename = "interAnnulusSpace")]
    pub inter_annulus_space: f64,
    #[serde(rename = "svgPrefix")]
    pub svg_prefix: String,
    #[serde(rename = "idDisplayed")]
    pub id_displayed: bool,
    #[serde(rename = "substationDescriptionDisplayed")]
    pub substation_description_displayed: bool,
    #[serde(rename = "arrowHeight")]
    pub arrow_height: f64,
    #[serde(rename = "busLegend")]
    pub bus_legend: bool,
    #[serde(rename = "voltageLevelDetails")]
    pub voltage_level_details: bool,
    #[serde(rename = "languageTag")]
    pub language_tag: String,
    #[serde(rename = "voltageValuePrecision")]
    pub voltage_value_precision: i32,
    #[serde(rename = "powerValuePrecision")]
    pub power_value_precision: i32,
    #[serde(rename = "angleValuePrecision")]
    pub angle_value_precision: i32,
    #[serde(rename = "currentValuePrecision")]
    pub current_value_precision: i32,
    #[serde(rename = "edgeInfoDisplayed")]
    pub edge_info_displayed: String,
    #[serde(rename = "pstArrowHeadSize")]
    pub pst_arrow_head_size: f64,
    #[serde(rename = "undefinedValueSymbol")]
    pub undefined_value_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BusNode {
    #[serde(rename = "svgId")]
    pub svg_id: String,
    #[serde(rename = "equipmentId")]
    pub equipment_id: String,
    #[serde(rename = "nbNeighbours")]
    pub nb_neighbours: i32,
    pub index: i32,
    #[serde(rename = "vlNode")]
    pub vl_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Node {
    #[serde(rename = "svgId")]
    pub svg_id: String,
    #[serde(rename = "equipmentId")]
    pub equipment_id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Edge {
    #[serde(rename = "svgId")]
    pub svg_id: String,
    #[serde(rename = "equipmentId")]
    pub equipment_id: String,
    pub node1: String,
    pub node2: String,
    #[serde(rename = "busNode1")]
    pub bus_node1: String,
    #[serde(rename = "busNode2")]
    pub bus_node2: String,
    #[serde(rename = "type")]
    pub edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TextNode {
    #[serde(rename = "svgId")]
    pub svg_id: String,
    #[serde(rename = "equipmentId")]
    pub equipment_id: String,
    #[serde(rename = "vlNode")]
    pub vl_node: String,
    #[serde(rename = "shiftX")]
    pub shift_x: f64,
    #[serde(rename = "shiftY")]
    pub shift_y: f64,
    #[serde(rename = "connectionShiftX")]
    pub connection_shift_x: f64,
    #[serde(rename = "connectionShiftY")]
    pub connection_shift_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetadataGrid {
    #[serde(rename = "layoutParameters")]
    pub layout_parameters: LayoutParameters,
    #[serde(rename = "svgParameters")]
    pub svg_parameters: SvgParameters,
    #[serde(rename = "busNodes")]
    pub bus_nodes: Vec<BusNode>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[serde(rename = "textNodes")]
    pub text_nodes: Vec<TextNode>,
}
