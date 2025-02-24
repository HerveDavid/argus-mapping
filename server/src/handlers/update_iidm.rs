use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bevy_ecs::event::Events;
use iidm::{Network, NetworkUpdate, UpdateEvent};
use serde::{Deserialize, Serialize};

use crate::states::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub id: String,
    pub component: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub status: String,
}

pub async fn update_iidm(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match try_update_component(&state, &payload).await {
        Ok(_) => (
            StatusCode::OK,
            Json(RegisterResponse {
                id: payload.id,
                status: "Component updated successfully".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(RegisterResponse {
                id: payload.id,
                status: format!("Failed to register component: {}", e),
            }),
        ),
    }
}

async fn try_update_component(
    state: &Arc<AppState>,
    payload: &RegisterRequest,
) -> Result<(), String> {
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;

    if let Ok(update) = serde_json::from_value::<NetworkUpdate>(payload.component.clone()) {
        let json = serde_json::to_string_pretty(&update).unwrap();
        println!("{}", json);

        // let id = payload.id.clone();
        // let mut event_writer = world.resource_mut::<Events<UpdateEvent<Network>>>();
        // event_writer.send(UpdateEvent { id, update });
        // schedule.run(&mut world);

        return Ok(());
    }

    Err("Could not deserialize component into any known type".to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    use iidm::TemporaryLimitUpdate;
    use serde::de::Error;
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
        let schema = T::fields();
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
        fn fields() -> Vec<String>;
        fn validate_json(json: &str) -> Result<Self, serde_json::Error>;
    }

    impl JsonSchema for TemporaryLimitUpdate {
        fn fields() -> Vec<String> {
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
}
