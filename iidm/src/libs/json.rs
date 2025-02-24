use serde::{de::Error, Deserialize};
use serde_json::Value;

pub fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: crate::extensions::JsonSchema + for<'de> Deserialize<'de> + schemars::JsonSchema,
{
    // Parse as Value for initial validation
    let value: Value = serde_json::from_str(json)
        .map_err(|e| serde_json::Error::custom(format!("Invalid JSON format: {}", e)))?;

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

    // Correct use of jsonschema
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

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
    struct TestUser {
        name: String,
        age: u32,
    }

    impl crate::extensions::JsonSchema for TestUser {
        type Err = String;

        fn fields_json() -> Vec<String> {
            vec!["name".to_string(), "age".to_string()]
        }

        fn validate_json(_json: &str) -> Result<Self, Self::Err> {
            unreachable!()
        }
    }

    #[test]
    fn test_valid_json() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            TestUser {
                name: "Alice".to_string(),
                age: 30,
            }
        );
    }

    #[test]
    fn test_invalid_json_format() {
        let json = r#"{"name": "Alice", "age": 30"#; // Missing closing brace
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid JSON format"));
    }

    #[test]
    fn test_non_object_json() {
        let json = r#""just a string""#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "JSON input must be an object"
        );
    }

    #[test]
    fn test_unexpected_field() {
        let json = r#"{"name": "Alice", "age": 30, "extra": "field"}"#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unexpected field: extra"));
    }

    #[test]
    fn test_schema_validation_failure() {
        let json = r#"{"name": "Alice", "age": -5}"#; // Age should be u32, not negative
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Schema validation failed"));
    }

    #[test]
    fn test_missing_required_field() {
        let json = r#"{"name": "Alice"}"#; // Missing age field
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Schema validation failed"));
    }

    #[test]
    fn test_wrong_type_field() {
        let json = r#"{"name": "Alice", "age": "thirty"}"#; // Age should be u32, not string
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Schema validation failed"));
    }

    #[test]
    fn test_empty_object() {
        let json = r#"{}"#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string() {
        let json = r#""#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid JSON format"));
    }

    #[test]
    fn test_array_instead_of_object() {
        let json = r#"[{"name": "Alice", "age": 30}]"#;
        let result: Result<TestUser, _> = validate_json(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "JSON input must be an object"
        );
    }
}
