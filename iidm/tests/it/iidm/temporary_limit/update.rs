use iidm::{JsonSchema, TemporaryLimitError, TemporaryLimitUpdater};

#[test]
fn test_update_from_not_json_updater() {
    let json = r#"{"name": "test", "acceptableDuration": 60"#; // Incorrect JSON syntax
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect JSON syntax"
    );
    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Invalid JSON"),
                "Error should indicate an invalid JSON issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
fn test_update_from_bad_json_updater() {
    let json = r#"{"nam": "coucou"}"#; // Incorrect key
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect key"
    );
    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
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
    let json = r#"{"Name": "test"}"#; // Incorrect case
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with an incorrect key case"
    );
    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
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
fn test_update_from_snake_case_key_json_updater() {
    let json = r#"{"acceptable_duration": 60}"#; // Snake case instead of camelCase
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with snake_case key instead of camelCase"
    );
    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
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
fn test_update_from_bad_value_type_json_updater() {
    let json = r#"{"acceptableDuration": "sixty"}"#; // String instead of number
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with incorrect value type"
    );
    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
            assert!(
                error.to_string().contains("Schema validation failed"),
                "Error should indicate a schema validation issue: {}",
                error
            );
        }
        _ => panic!("Expected a Deserialization error"),
    }
}

#[test]
#[ignore = "need to specify acceptableDuration"]
fn test_update_from_negative_acceptable_duration() {
    let json = r#"{"acceptableDuration": -60}"#; // Negative duration
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with negative acceptableDuration"
    );
    // Note: This assumes schema validation rejects negative durations.
    // If your schema allows negative values, adjust this test accordingly.
}

#[test]
fn test_update_from_good_json_updater() {
    let json = r#"{"name": "coucou"}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with a valid JSON"
    );
    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "coucou");
    }
}

#[test]
fn test_update_from_missing_json_updater() {
    let json = r#"{"name": "coucou"}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with missing fields"
    );
    if let Ok(validated) = validation {
        assert!(validated.value.is_none(), "Value should be None");
        assert!(
            validated.acceptable_duration.is_none(),
            "AcceptableDuration should be None"
        );
    }
}

#[test]
fn test_update_from_null_json_updater() {
    let json = r#"{"name": "coucou", "value": null}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with null values"
    );
    if let Ok(validated) = validation {
        assert!(validated.value.is_none(), "Value should be None");
    }
}

#[test]
fn test_update_all_fields_json_updater() {
    let json = r#"{"name": "test-limit", "acceptableDuration": 300, "value": 1200.5}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with all fields provided: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "test-limit");
        assert_eq!(validated.acceptable_duration.unwrap(), 300);
        assert_eq!(validated.value.unwrap(), 1200.5);
    }
}

#[test]
fn test_update_with_floating_point_acceptable_duration() {
    let json = r#"{"acceptableDuration": 60.5}"#; // Floating point instead of integer
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with floating point acceptableDuration"
    );
    // This assumes schema validation enforces integer type for acceptableDuration
}

#[test]
fn test_update_with_empty_name() {
    let json = r#"{"name": ""}"#; // Empty string
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with empty name: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "");
    }
}

#[test]
fn test_update_with_very_large_value() {
    let json = r#"{"value": 1e308}"#; // Very large number
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with very large value: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert!(validated.value.is_some(), "Value should be Some");
        assert!(
            validated.value.unwrap() > 1e300,
            "Value should be very large"
        );
    }
}

#[test]
fn test_update_with_very_large_duration() {
    let json = r#"{"acceptableDuration": 2147483647}"#; // Max i32 value
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with max i32 value: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.acceptable_duration.unwrap(), 2147483647);
    }
}

#[test]
fn test_update_with_additional_fields() {
    let json = r#"{"name": "test", "acceptableDuration": 60, "value": 100.0, "extraField": "should be ignored"}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "Validation should fail with additional unknown fields"
    );

    match validation.err().unwrap() {
        TemporaryLimitError::Deserialization(error) => {
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
fn test_update_with_zero_values() {
    let json = r#"{"acceptableDuration": 0, "value": 0.0}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with zero values: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.acceptable_duration.unwrap(), 0);
        assert_eq!(validated.value.unwrap(), 0.0);
    }
}

#[test]
fn test_update_with_special_characters_in_name() {
    let json = r#"{"name": "特殊字符-!@#$%^&*()"}"#; // Special characters
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_ok(),
        "Validation should succeed with special characters in name: {:?}",
        validation.err()
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "特殊字符-!@#$%^&*()");
    }
}
