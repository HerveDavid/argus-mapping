// use iidm::{TemporaryLimitError, TemporaryLimitUpdater};
// use serde::{de::Error, Deserialize, Serialize};
// use serde_json::Value;

// #[test]
// fn test_update_from_bad_json_updater() {
//     let json = r#"{"nam": "coucou"}"#; // Incorrect key
//     let validation = TemporaryLimitUpdater::validate_json(json);
//     assert!(
//         validation.is_err(),
//         "Validation should fail with an incorrect key"
//     );
//     match validation.err().unwrap() {
//         TemporaryLimitError::Deserialization(error) => {
//             assert!(
//                 error.to_string().contains("Unexpected field"),
//                 "Error should indicate an unexpected field issue: {}",
//                 error
//             );
//         }
//         _ => panic!("Expected a Deserialization error"),
//     }
// }

// #[test]
// fn test_update_from_good_json_updater() {
//     let json = r#"{"name": "coucou"}"#;
//     let validation = TemporaryLimitUpdater::validate_json(json);

//     assert!(
//         validation.is_ok(),
//         "Validation should sucess with a JSON valid"
//     );

//     if let Ok(validated) = validation {
//         assert_eq!(validated.name.unwrap(), "coucou");
//     }
// }

// #[test]
// fn test_update_from_missing_json_updater() {
//     let json = r#"{"name": "coucou"}"#;
//     let validation = TemporaryLimitUpdater::validate_json(json);
//     assert!(validation.is_ok());
//     if let Ok(validated) = validation {
//         assert!(validated.value.is_none(), "Value should be None");
//     }
// }

// #[test]
// fn test_update_from_null_json_updater() {
//     let json = r#"{"name": "coucou", "value": null}"#;
//     let validation = TemporaryLimitUpdater::validate_json(json);
//     assert!(validation.is_ok());
//     if let Ok(validated) = validation {
//         assert!(validated.value.is_none(), "Value should be None");
//     }
// }

// pub fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
// where
//     T: JsonSchema + Serialize + for<'de> Deserialize<'de>,
// {
//     let value: Value = serde_json::from_str(json)?;

//     // Vérification stricte que c'est un objet
//     let obj = value
//         .as_object()
//         .ok_or_else(|| serde_json::Error::custom("JSON input must be an object"))?;

//     // Vérification des champs attendus
//     let schema = T::fields_json();
//     for field in obj.keys() {
//         if !schema.contains(field) {
//             return Err(serde_json::Error::custom(format!(
//                 "Unexpected field: {}",
//                 field
//             )));
//         }
//     }

//     let inner: T = serde_json::from_value(Value::Object(obj.clone()))?;

//     Ok(inner)
// }

// // Implémentation du trait pour définir le schéma JSON attendu
// pub trait JsonSchema: for<'de> Deserialize<'de> {
//     type Err;

//     fn fields_json() -> Vec<String>;
//     fn validate_json(json: &str) -> Result<Self, Self::Err>;
// }

// impl JsonSchema for TemporaryLimitUpdater {
//     type Err = TemporaryLimitError;

//     fn fields_json() -> Vec<String> {
//         vec![
//             "name".to_string(),
//             "acceptableDuration".to_string(),
//             "value".to_string(),
//         ]
//     }

//     fn validate_json(json: &str) -> Result<Self, Self::Err> {
//         validate_json(json).map_err(|e| TemporaryLimitError::Deserialization(e))
//     }
// }
