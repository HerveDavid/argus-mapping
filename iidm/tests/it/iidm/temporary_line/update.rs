use iidm::TemporaryLimitUpdater;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

#[test]
fn test_update_from_bad_json_updater() {
    let json = r#"{"nam": "coucou"}"#; // Incorrecte key
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(
        validation.is_err(),
        "La validation devrait échouer avec une clé incorrecte"
    );
}

#[test]
fn test_update_from_good_json_updater() {
    let json = r#"{"name": "coucou"}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);

    assert!(
        validation.is_ok(),
        "La validation devrait réussir avec un JSON valide"
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "coucou");
    }
}

#[test]
fn test_update_from_missing_json_updater() {
    let json = r#"{"name": "coucou"}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(validation.is_ok());
    if let Ok(validated) = validation {
        assert!(validated.value.is_none(), "La valeur devrait être None");
    }
}

#[test]
fn test_update_from_null_json_updater() {
    let json = r#"{"name": "coucou", "value": null}"#;
    let validation = TemporaryLimitUpdater::validate_json(json);
    assert!(validation.is_ok());
    if let Ok(validated) = validation {
        assert!(validated.value.is_none(), "La valeur devrait être None");
    }
}

pub fn validate_json<T>(json: &str) -> Result<T, serde_json::Error>
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

// Implémentation du trait pour définir le schéma JSON attendu
pub trait JsonSchema: Sized {
    fn fields_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, serde_json::Error>;
}

impl JsonSchema for TemporaryLimitUpdater {
    fn fields_json() -> Vec<String> {
        vec![
            "name".to_string(),
            "acceptable_duration".to_string(),
            "value".to_string(),
        ]
    }

    fn validate_json(json: &str) -> Result<Self, serde_json::Error> {
        validate_json(json)
    }
}
