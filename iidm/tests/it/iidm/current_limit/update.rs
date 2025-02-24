use iidm::{CurrentLimitsError, CurrentLimitsUpdater, TemporaryLimitError, TemporaryLimitUpdater};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

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

// Trait to define the expected JSON schema
trait JsonSchema: for<'de> Deserialize<'de> + Serialize {
    type Err;
    fn fields_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, Self::Err>;
}

fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: JsonSchema + for<'de> Deserialize<'de> + schemars::JsonSchema,
{
    // Parse as Value for initial validation
    let value: Value = serde_json::from_str(json)?;

    // Make sure it's an object
    let obj = value
        .as_object()
        .ok_or_else(|| serde_json::Error::custom("JSON input must be an object"))?;

    // Check for unexpected fields
    let schema_fields = T::fields_json();
    for field in obj.keys() {
        if !schema_fields.contains(&field.to_string()) {
            return Err(serde_json::Error::custom(format!(
                "Unexpected field: {}",
                field
            )));
        }
    }

    // Get the JSON schema for T
    let schema = schemars::schema_for!(T);
    let schema_value = serde_json::to_value(&schema).map_err(|e| {
        serde_json::Error::custom(format!("Failed to convert schema to value: {}", e))
    })?;

    // Utilisation correcte de jsonschema
    let instance = &value;
    let result = jsonschema::validate(&schema_value, instance);

    if let Err(errors) = result {
        return Err(serde_json::Error::custom(format!(
            "Schema validation failed: {}",
            errors
        )));
    }

    // If validation passes, deserialize the input
    serde_json::from_value(value)
}

impl JsonSchema for TemporaryLimitUpdater {
    type Err = TemporaryLimitError;

    fn fields_json() -> Vec<String> {
        vec![
            "name".to_string(),
            "acceptableDuration".to_string(),
            "value".to_string(),
        ]
    }

    fn validate_json(json: &str) -> Result<Self, Self::Err> {
        validate_json(json).map_err(TemporaryLimitError::Deserialization)
    }
}

impl JsonSchema for CurrentLimitsUpdater {
    type Err = CurrentLimitsError;

    fn fields_json() -> Vec<String> {
        vec!["permanentLimit".to_string(), "temporaryLimits".to_string()]
    }

    fn validate_json(json: &str) -> Result<Self, Self::Err> {
        validate_json(json).map_err(CurrentLimitsError::Deserialization)
    }
}
