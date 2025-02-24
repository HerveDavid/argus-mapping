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
