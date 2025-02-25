use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::error;

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

    #[error("{0}")]
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
// Dispatcher function
pub async fn update_iidm(
    Path(component_type): Path<String>,
    state: State<Arc<AppState>>,
    payload: Json<RegisterRequest>,
) -> Result<Response, UpdateError> {
    let ecs = state.ecs.read().await;
    let update_registry = ecs.update_registry.read().await;

    // Get the appropriate handler
    let handler = update_registry
        .get_handler(&component_type)
        .ok_or_else(|| {
            UpdateError::NotFoundError(format!(
                "No handler registered for component type: {}",
                component_type
            ))
        })?;

    // Call the handler with the original state and payload
    handler(state.clone(), payload).await
}
