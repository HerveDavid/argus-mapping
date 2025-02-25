#[cfg(test)]
mod property_tests {
    use iidm::libs::json::validate_json;
    use proptest::prelude::*;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
    struct TestPerson {
        name: String,
        age: u32,
        email: Option<String>,
    }

    impl iidm::JsonSchema for TestPerson {
        type Err = String;
        fn fields_json() -> Vec<String> {
            vec!["name".to_string(), "age".to_string(), "email".to_string()]
        }
        fn validate_json(_json: &str) -> Result<Self, Self::Err> {
            unreachable!()
        }
    }

    // Strategy to generate valid TestPerson objects
    fn valid_person_strategy() -> impl Strategy<Value = TestPerson> {
        (
            // Names between 1 and 50 characters
            "[a-zA-Z0-9 ]{1,50}",
            // Age between 0 and 120
            0..120u32,
            // Optional email
            proptest::option::of("[a-zA-Z0-9]{1,20}@[a-zA-Z0-9]{1,20}\\.[a-z]{2,5}"),
        )
            .prop_map(|(name, age, email)| TestPerson { name, age, email })
    }

    proptest! {
        // Test that valid JSON objects are correctly deserialized
        #[test]
        fn test_valid_json_objects_deserialize(person in valid_person_strategy()) {
            let json = serde_json::to_string(&person).unwrap();
            let result: Result<TestPerson, _> = validate_json(&json);
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap(), person);
        }

        // Test with JSON objects containing additional fields
        #[test]
        fn test_extra_fields_fail(
            person in valid_person_strategy(),
            extra_field in "[a-zA-Z]{1,10}",
            extra_value in "[a-zA-Z0-9]{1,20}"
        ) {
            // Skip if the extra field is one of the valid fields
            prop_assume!(extra_field != "name" && extra_field != "age" && extra_field != "email");

            // Create a JSON object with an extra field
            let mut obj = serde_json::to_value(&person).unwrap();
            if let serde_json::Value::Object(ref mut map) = obj {
                map.insert(extra_field.clone(), serde_json::Value::String(extra_value));
            }

            let json = serde_json::to_string(&obj).unwrap();
            let result: Result<TestPerson, _> = validate_json(&json);

            prop_assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            let expected_message = format!("Unexpected field: {}", &extra_field);
            prop_assert!(err.contains(&expected_message));
        }

        // Test with non-object JSON
        #[test]
        fn test_non_objects_fail(value in ".*") {
            // Try to create a JSON that is not an object
            let json = format!("\"{}\"", value.replace("\"", "\\\""));

            let result: Result<TestPerson, _> = validate_json(&json);
            prop_assert!(result.is_err());

            let err = result.unwrap_err().to_string();
            prop_assert!(err == "JSON input must be an object" || err.contains("Invalid JSON format"));
        }

        // Test with invalid ages (beyond the limit of u32)
        #[test]
        fn test_invalid_age_type(
            name in "[a-zA-Z0-9 ]{1,50}",
            age_str in "[a-zA-Z]{1,10}",
        ) {
            let json = format!(r#"{{"name":"{}", "age":"{}"}}"#, name, age_str);
            let result: Result<TestPerson, _> = validate_json(&json);

            prop_assert!(result.is_err());
            prop_assert!(result.unwrap_err().to_string().contains("Schema validation failed"));
        }

        // Test with incorrect JSON formats
        #[test]
        fn test_invalid_json_syntax(json in "[^{}]*") {
            // Verify that the JSON is not already valid by chance
            prop_assume!(!json.starts_with("{") || !json.ends_with("}"));

            let result: Result<TestPerson, _> = validate_json(&json);
            prop_assert!(result.is_err());
        }
    }
}
