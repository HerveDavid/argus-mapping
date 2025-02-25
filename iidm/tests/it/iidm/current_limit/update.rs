use iidm::{CurrentLimitsError, CurrentLimitsUpdater, JsonSchema, TemporaryLimitUpdater};

#[test]
fn test_update_from_not_json_updater() {
    let json = r#"{"permanentLimits": 1.0"#; // Incorrect json
    let validation = CurrentLimitsUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect json"
    );
    match validation.err().unwrap() {
        CurrentLimitsError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Invalid JSON"),
                "Error should indicate an unexpected field issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_from_bad_key_json_updater() {
    let json = r#"{"permanentLimits": 1.0}"#; // Incorrect key
    let validation = CurrentLimitsUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect key"
    );
    match validation.err().unwrap() {
        CurrentLimitsError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Unexpected field"),
                "Error should indicate an unexpected field issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_from_bad_case_key_json_updater() {
    let json = r#"{"permanent_limit": 1}"#; // Incorrect key type
    let validation = CurrentLimitsUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect value type"
    );
    match validation.err().unwrap() {
        CurrentLimitsError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Unexpected field"),
                "Error should indicate an unexpected field issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_from_bad_value_json_updater() {
    let json = r#"{"permanentLimit": "a string"}"#; // Incorrect value type
    let validation = CurrentLimitsUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect value type"
    );
    match validation.err().unwrap() {
        CurrentLimitsError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Schema validation failed"),
                "Error should indicate an unexpected field issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_from_good_json_updater() {
    let json = r#"{"permanentLimit": 1.0}"#;
    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with a valid JSON: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert!(
            validated.permanent_limit.is_some(),
            "permanent_limit should be Some"
        );
        assert_eq!(validated.permanent_limit.unwrap(), 1.0);
    }
}

#[test]
fn test_update_from_empty_temporary_limits() {
    let json = r#"{"permanentLimit": 1.0, "temporaryLimits": []}"#;
    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with empty temporaryLimits array: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert!(
            validated.permanent_limit.is_some(),
            "permanent_limit should be Some"
        );
        assert_eq!(validated.permanent_limit.unwrap(), 1.0);
        assert!(
            validated.temporary_limits.is_some(),
            "temporary_limits should be Some"
        );
        assert!(
            validated.temporary_limits.unwrap().is_empty(),
            "temporary_limits should be empty"
        );
    }
}

#[test]
fn test_update_from_bad_temporary_limit_items() {
    let json = r#"{"permanentLimit": 1.0, "temporaryLimits": ["not an object"]}"#;
    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_err(),
        "Validation should fail with invalid temporaryLimits items"
    );

    match validation.err().unwrap() {
        CurrentLimitsError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Schema validation failed"),
                "Error should indicate schema validation failure: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_with_valid_temporary_limits() {
    let json = r#"{
        "permanentLimit": 1.0, 
        "temporaryLimits": [
            {"name": "limit1", "acceptableDuration": 60, "value": 1.2},
            {"name": "limit2", "acceptableDuration": 300, "value": 1.5}
        ]
    }"#;

    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with valid temporaryLimits: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert!(
            validated.permanent_limit.is_some(),
            "permanent_limit should be Some"
        );
        assert_eq!(validated.permanent_limit.unwrap(), 1.0);

        assert!(
            validated.temporary_limits.is_some(),
            "temporary_limits should be Some"
        );

        let limits = validated.temporary_limits.unwrap();
        assert_eq!(limits.len(), 2, "Should have 2 temporary limits");

        assert_eq!(limits[0].name, "limit1");
        assert_eq!(limits[0].acceptable_duration, 60);
        assert_eq!(limits[0].value, 1.2);

        assert_eq!(limits[1].name, "limit2");
        assert_eq!(limits[1].acceptable_duration, 300);
        assert_eq!(limits[1].value, 1.5);
    }
}

#[test]
fn test_temporary_limit_updater_validation() {
    // Test avec un JSON valide
    let json = r#"{"name": "test", "acceptableDuration": 60, "value": 1.2}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with valid JSON: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.as_ref().unwrap(), "test");
        assert_eq!(validated.acceptable_duration.unwrap(), 60);
        assert_eq!(validated.value.unwrap(), 1.2);
    }

    // Test avec un champ manquant
    let json = r#"{"name": "test", "value": 1.2}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with missing field because all fields are Option: {:?}",
        validation.err()
    );

    // Test avec un type incorrect
    let json = r#"{"name": "test", "acceptableDuration": "not a number", "value": 1.2}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);

    assert!(
        validation.is_err(),
        "Validation should fail with incorrect type"
    );
}

#[test]
fn test_partial_updates() {
    // Test avec seulement permanentLimit
    let json = r#"{"permanentLimit": 2.0}"#;
    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with partial update: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.permanent_limit.unwrap(), 2.0);
        assert!(validated.temporary_limits.is_none());
    }

    // Test avec seulement temporaryLimits
    let json =
        r#"{"temporaryLimits": [{"name": "limit1", "acceptableDuration": 60, "value": 1.2}]}"#;
    let validation = CurrentLimitsUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "Validation should succeed with partial update: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert!(validated.permanent_limit.is_none());
        assert!(validated.temporary_limits.is_some());
        assert_eq!(validated.temporary_limits.unwrap().len(), 1);
    }
}
