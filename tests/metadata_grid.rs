use argus_mapping::MetadataGrid;
use serde_json::from_str;
use std::fs;

#[test]
fn test_metadata_grid_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Load test data from file
    let data = fs::read_to_string("tests/data/simple-eu.json")?;
    let grid: MetadataGrid = from_str(&data)?;

    // 1. Basic structure validation
    assert!(!grid.bus_nodes.is_empty(), "Bus nodes should not be empty");
    assert!(!grid.nodes.is_empty(), "Nodes should not be empty");
    assert!(!grid.edges.is_empty(), "Edges should not be empty");
    assert!(
        !grid.text_nodes.is_empty(),
        "Text nodes should not be empty"
    );

    // 2. Verify layout parameters
    assert!(
        !grid.layout_parameters.text_nodes_force_layout,
        "Force layout should be disabled by default"
    );
    assert_eq!(
        grid.layout_parameters.max_steps, 1000,
        "Max steps should be 1000"
    );

    // 3. Verify SVG parameters
    assert_eq!(
        grid.svg_parameters.css_location, "INSERTED_IN_SVG",
        "CSS location should be INSERTED_IN_SVG"
    );
    assert_eq!(
        grid.svg_parameters.size_constraint, "FIXED_SCALE",
        "Size constraint should be FIXED_SCALE"
    );

    // 4. Test node connectivity
    let edge_count = grid.edges.len();
    let unique_nodes: std::collections::HashSet<_> = grid
        .edges
        .iter()
        .flat_map(|edge| vec![&edge.node1, &edge.node2])
        .collect();

    assert!(edge_count > 0, "Should have edges connecting nodes");
    assert!(unique_nodes.len() > 0, "Should have connected nodes");

    // 5. Verify coordinates exist and find bounds
    let (mut min_x, mut max_x) = (f64::MAX, f64::MIN);
    let (mut min_y, mut max_y) = (f64::MAX, f64::MIN);

    for node in &grid.nodes {
        min_x = min_x.min(node.x);
        max_x = max_x.max(node.x);
        min_y = min_y.min(node.y);
        max_y = max_y.max(node.y);

        // Basic sanity checks that coordinates are finite
        assert!(node.x.is_finite(), "X coordinate must be finite");
        assert!(node.y.is_finite(), "Y coordinate must be finite");
    }

    println!(
        "Coordinate bounds: X: [{}, {}], Y: [{}, {}]",
        min_x, max_x, min_y, max_y
    );

    // 6. Check text node properties
    for text_node in &grid.text_nodes {
        assert_eq!(
            text_node.shift_x, 100.0,
            "Text node X shift should be 100.0"
        );
        assert_eq!(
            text_node.shift_y, -40.0,
            "Text node Y shift should be -40.0"
        );
        assert!(
            !text_node.vl_node.is_empty(),
            "VL node reference should not be empty"
        );
    }

    // 7. Verify edge consistency
    for edge in &grid.edges {
        // Every edge should have both node references
        assert!(!edge.node1.is_empty(), "Edge node1 should not be empty");
        assert!(!edge.node2.is_empty(), "Edge node2 should not be empty");
        assert!(
            !edge.bus_node1.is_empty(),
            "Edge bus_node1 should not be empty"
        );
        assert!(
            !edge.bus_node2.is_empty(),
            "Edge bus_node2 should not be empty"
        );
        assert!(!edge.edge_type.is_empty(), "Edge type should not be empty");
    }

    // 8. Verify bus node references
    for bus_node in &grid.bus_nodes {
        assert!(
            bus_node.nb_neighbours >= 0,
            "Number of neighbours should be non-negative"
        );
        assert!(bus_node.index >= 0, "Bus node index should be non-negative");
    }

    // 9. Test padding values
    assert!(grid.svg_parameters.diagram_padding.left > 0.0);
    assert!(grid.svg_parameters.diagram_padding.right > 0.0);
    assert!(grid.svg_parameters.diagram_padding.top > 0.0);
    assert!(grid.svg_parameters.diagram_padding.bottom > 0.0);

    Ok(())
}

#[test]
fn test_edge_types() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("tests/data/simple-eu.json")?;
    let grid: MetadataGrid = from_str(&data)?;

    let edge_types: std::collections::HashSet<String> = grid
        .edges
        .iter()
        .map(|edge| edge.edge_type.clone())
        .collect();

    // Verify expected edge types are present
    assert!(edge_types.contains("LineEdge"), "Should have LineEdge type");
    assert!(
        edge_types.contains("TwoWtEdge"),
        "Should have TwoWtEdge type"
    );

    Ok(())
}

#[test]
fn test_node_uniqueness() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("tests/data/simple-eu.json")?;
    let grid: MetadataGrid = from_str(&data)?;

    // Check for unique SVG IDs
    let mut svg_ids = std::collections::HashSet::new();
    for node in &grid.nodes {
        assert!(
            svg_ids.insert(&node.svg_id),
            "Duplicate SVG ID found: {}",
            node.svg_id
        );
    }

    // Check for unique equipment IDs
    let mut equipment_ids = std::collections::HashSet::new();
    for node in &grid.nodes {
        assert!(
            equipment_ids.insert(&node.equipment_id),
            "Duplicate equipment ID found: {}",
            node.equipment_id
        );
    }

    Ok(())
}
