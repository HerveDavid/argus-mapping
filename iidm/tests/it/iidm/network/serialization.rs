use super::*;
use iidm::*;

const NETWORK_FILE: &str = "tests/data/network.json";

#[test]
fn test_deserialize_from_json() {
    let network: Network = serde_json::from_str(VALID_NETWORK_JSON).unwrap();
    assert_default_values(&network);
}

#[test]
fn test_serialize_to_json() {
    let network = create_default_network();
    let json = serde_json::to_string(&network).unwrap();
    let deserialized: Network = serde_json::from_str(&json).unwrap();
    assert_default_values(&deserialized);
}

#[test]
fn test_network_basic_properties() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string(NETWORK_FILE)?;
    let network: Network = serde_json::from_str(&test_network)?;

    assert_eq!(network.version, "1.12");
    assert_eq!(network.id, "sim1");
    assert_eq!(network.forecast_distance, 0);
    assert_eq!(network.source_format, "test");
    assert_eq!(network.minimum_validation_level, "STEADY_STATE_HYPOTHESIS");

    Ok(())
}

#[test]
fn test_network_substations() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string(NETWORK_FILE)?;
    let network: Network = serde_json::from_str(&test_network)?;

    assert_eq!(network.substations.len(), 2);

    let p1 = &network.substations[0];
    assert_eq!(p1.id, "P1");
    assert_eq!(p1.country, "FR");
    assert_eq!(p1.tso, "RTE");
    assert_eq!(p1.geographical_tags, vec!["A"]);

    // Vérifie les niveaux de tension de P1
    assert_eq!(p1.voltage_levels.len(), 2);
    let vlgen = &p1.voltage_levels[0];
    assert_eq!(vlgen.id, "VLGEN");
    assert_eq!(vlgen.nominal_v, 24.0);
    assert_eq!(vlgen.topology_kind, TopologyKind::BusBreaker);

    // Vérifie le générateur
    if let Some(generators) = &vlgen.generators {
        assert_eq!(generators.len(), 1);
        let gen = &generators[0];
        assert_eq!(gen.id, "GEN");
        assert_eq!(gen.energy_source, EnergySource::Other);
        assert_eq!(gen.target_p, 607.0);
        assert_eq!(gen.target_v, 24.5);
    }

    Ok(())
}

#[test]
fn test_network_lines() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string("tests/data/network.json")?;
    let network: Network = serde_json::from_str(&test_network)?;

    assert_eq!(network.lines.len(), 2);

    let line1 = &network.lines[0];
    assert_eq!(line1.id, "NHV1_NHV2_1");
    assert_eq!(line1.r, 3.0);
    assert_eq!(line1.x, 33.0);
    assert_eq!(line1.voltage_level_id1, "VLHV1");
    assert_eq!(line1.voltage_level_id2, "VLHV2");

    Ok(())
}

#[test]
fn test_network_transformers() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string("tests/data/network.json")?;
    let network: Network = serde_json::from_str(&test_network)?;

    let p1 = &network.substations[0];
    assert_eq!(p1.two_windings_transformers.len(), 1);
    let transformer = &p1.two_windings_transformers[0];
    assert_eq!(transformer.id, "NGEN_NHV1");
    assert_eq!(transformer.rated_u1, 24.0);
    assert_eq!(transformer.rated_u2, 400.0);

    let p2 = &network.substations[1];
    let transformer = &p2.two_windings_transformers[0];
    assert_eq!(transformer.id, "NHV2_NLOAD");
    if let Some(tap_changer) = &transformer.ratio_tap_changer {
        assert!(tap_changer.regulating);
        assert_eq!(tap_changer.tap_position, 1);
        assert_eq!(tap_changer.steps.len(), 3);
    }

    Ok(())
}

#[test]
fn test_network_loads() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string("tests/data/network.json")?;
    let network: Network = serde_json::from_str(&test_network)?;

    let p2 = &network.substations[1];
    let vlload = &p2.voltage_levels[1];
    if let Some(loads) = &vlload.loads {
        assert_eq!(loads.len(), 1);
        let load = &loads[0];
        assert_eq!(load.id, "LOAD");
        assert_eq!(load.load_type, LoadType::Undefined);
        assert_eq!(load.p0, 600.0);
        assert_eq!(load.q0, 200.0);
    }

    Ok(())
}

#[test]
fn test_serialization_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string("tests/data/network.json")?;
    let network: Network = serde_json::from_str(&test_network)?;
    let serialized = serde_json::to_string(&network)?;
    let deserialized: Network = serde_json::from_str(&serialized)?;

    assert_eq!(network.id, deserialized.id);
    assert_eq!(network.version, deserialized.version);
    assert_eq!(network.substations.len(), deserialized.substations.len());
    assert_eq!(network.lines.len(), deserialized.lines.len());

    Ok(())
}
