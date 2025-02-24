use iidm::TemporaryLimitUpdate;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

#[test]
fn test() {
    // Test 1: JSON mal formé (devrait échouer)
    let json = r#"{"nam": "coucou"}"#; // Clé incorrecte
    let validation = TemporaryLimitUpdate::validate_json(json);
    assert!(
        validation.is_err(),
        "La validation devrait échouer avec une clé incorrecte"
    );

    // Test 2: JSON bien formé
    let json = r#"{"name": "coucou"}"#;
    let validation = TemporaryLimitUpdate::validate_json(json);
    assert!(
        validation.is_ok(),
        "La validation devrait réussir avec un JSON valide"
    );

    if let Ok(validated) = validation {
        assert_eq!(validated.name.unwrap(), "coucou");
    }

    // Test 3: Test avec des champs manquants
    let json = r#"{"name": "coucou", "value": null}"#;
    let validation = TemporaryLimitUpdate::validate_json(json);
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
    let schema = T::schema_json();
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
    fn schema_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, serde_json::Error>;
}

impl JsonSchema for TemporaryLimitUpdate {
    fn schema_json() -> Vec<String> {
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
