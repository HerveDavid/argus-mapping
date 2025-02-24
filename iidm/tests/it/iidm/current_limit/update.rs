use iidm::{CurrentLimitsError, CurrentLimitsUpdater, TemporaryLimitError, TemporaryLimitUpdater};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

#[test]
fn test_update_from_bad_json_updater() {
    let json = r#"{"permanent_limits": "coucou"}"#; // Incorrect key
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
fn test_update_from_bad_number_json_updater() {
    let json = r#"{"permanent_limits": "coucou"}"#; // Incorrecte key
    let validation = CurrentLimitsUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "La validation devrait échouer avec une clé incorrecte"
    );
}

// Implémentation du trait pour définir le schéma JSON attendu
trait JsonSchema: for<'de> Deserialize<'de> {
    type Err;

    fn fields_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, Self::Err>;
}

fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: JsonSchema + Serialize + for<'de> Deserialize<'de>,
{
    let value: Value = serde_json::from_str(json)?;

    // Vérification stricte que c'est un objet
    let obj = value
        .as_object()
        .ok_or_else(|| serde_json::Error::custom("JSON input must be an object"))?;

    // Vérification des champs attendus
    let schema = T::fields_json();
    for field in obj.keys() {
        if !schema.contains(field) {
            return Err(serde_json::Error::custom(format!(
                "Unexpected field: {}",
                field
            )));
        }
    }

    let inner: T = serde_json::from_value(Value::Object(obj.clone()))?;

    Ok(inner)
}

impl JsonSchema for TemporaryLimitUpdater {
    type Err = TemporaryLimitError;

    fn fields_json() -> Vec<String> {
        vec![
            "name".to_string(),
            "acceptable_duration".to_string(),
            "value".to_string(),
        ]
    }

    fn validate_json(json: &str) -> Result<Self, Self::Err> {
        validate_json(json).map_err(|e| TemporaryLimitError::Deserialization(e))
    }
}

impl JsonSchema for CurrentLimitsUpdater {
    type Err = CurrentLimitsError;

    fn fields_json() -> Vec<String> {
        vec![
            "permanent_limit".to_string(),
            "temporary_limits".to_string(),
        ]
    }

    fn validate_json(json: &str) -> Result<Self, Self::Err> {
        validate_json(json).map_err(|e| CurrentLimitsError::Deserialization(e))
    }
}
