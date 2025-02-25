use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bevy_ecs::event::Events;
use iidm::{EntityNotFoundEvent, ErrorType, JsonSchema, Updatable, UpdateEvent};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use thiserror::Error;
use tracing::{debug, error, instrument};

use crate::states::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub id: String,
    pub component: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub status: String,
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Failed to parse JSON: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid network data: {0}")]
    ValidationError(String),

    #[error("Component not found: {0}")]
    NotFoundError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for UpdateError {
    fn into_response(self) -> Response {
        let status = match self {
            UpdateError::SerializationError(_) | UpdateError::ValidationError(_) => {
                StatusCode::BAD_REQUEST
            }
            UpdateError::NotFoundError(_) => StatusCode::NOT_FOUND,
            UpdateError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(RegisterResponse {
            status: self.to_string(),
        });

        (status, body).into_response()
    }
}

#[instrument(skip(state, payload), fields(id = %payload.id))]
pub async fn update_iidm<C, U, E>(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, UpdateError>
where
    C: Updatable<Updater = U, Err = E> + 'static,
    U: JsonSchema + Send + Sync + 'static,
    E: Display,
    U::Err: Display,
{
    debug!("Received update request for component ID: {}", payload.id);

    update_component::<C, U, E>(&state, &payload).await?;

    Ok((
        StatusCode::OK,
        Json(RegisterResponse {
            status: "Component updated successfully".to_string(),
        }),
    ))
}

async fn update_component<C, U, E>(
    state: &Arc<AppState>,
    payload: &RegisterRequest,
) -> Result<(), UpdateError>
where
    C: Updatable<Updater = U, Err = E> + 'static,
    U: JsonSchema + Send + Sync + 'static,
    E: Display,
    U::Err: Display,
{
    let ecs = state.ecs.read().await;

    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;
    let id = payload.id.clone();

    // Convert the component JSON to a string
    let json_str = serde_json::to_string(&payload.component)?;

    // Validate the JSON against the component schema
    let update =
        U::validate_json(&json_str).map_err(|e| UpdateError::ValidationError(e.to_string()))?;

    // Send the update event
    match world.get_resource_mut::<Events<UpdateEvent<C>>>() {
        Some(mut event_writer) => {
            event_writer.send(UpdateEvent {
                id: id.clone(),
                update,
            });

            // Run the schedule to process the event
            schedule.run(&mut world);

            // Check if any error events were generated
            let error_events = world.resource_mut::<Events<EntityNotFoundEvent>>();
            let mut error_reader = error_events.get_cursor();

            for error in error_reader.read(&error_events) {
                if error.id == id {
                    match error.error_type {
                        ErrorType::EntityNotFound => {
                            return Err(UpdateError::NotFoundError(format!(
                                "Entity with ID '{}' not found",
                                id
                            )));
                        }
                        ErrorType::ComponentNotFound => {
                            return Err(UpdateError::NotFoundError(format!(
                                "Component of type '{}' not found on entity with ID '{}'",
                                error.component_type, id
                            )));
                        }
                    }
                }
            }

            debug!("Successfully updated component: {}", id);
            Ok(())
        }
        None => {
            error!("Events resource not found");
            Err(UpdateError::InternalError(
                "Event system not initialized".to_string(),
            ))
        }
    }
}
