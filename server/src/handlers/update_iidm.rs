use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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
    Err("Could not deserialize component into any known type".to_string())
}
