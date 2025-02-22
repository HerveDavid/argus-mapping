use crate::states::AppState;
use askama::Template;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use bevy_ecs::event::Events;
use iidm::{Identifiable, Network, RegisterEvent};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("Multipart field error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Template rendering error: {0}")]
    TemplateError(#[from] askama::Error),
    #[error("No IIDM file provided")]
    NoFile,
}

// Implement IntoResponse for our error type
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        (StatusCode::BAD_REQUEST, message).into_response()
    }
}

#[derive(Template)]
#[template(path = "iidm_table.html")]
struct IIdmTableTemplate {
    message: String,
    network: Option<Network>,
}

impl IIdmTableTemplate {
    fn new(message: String, network: Option<Network>) -> Self {
        Self { message, network }
    }
}

pub async fn upload_iidm(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let network = process_upload(&mut multipart).await?;

    // Update ECS state
    update_ecs_state(&state, &network).await;

    let iidm_table = serde_json::to_string_pretty(&network).map_err(UploadError::JsonError)?;

    let template = IIdmTableTemplate::new(iidm_table, Some(network));
    let html = template.render().map_err(UploadError::TemplateError)?;

    Ok(Html(html))
}

async fn process_upload(multipart: &mut Multipart) -> Result<Network, UploadError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(UploadError::MultipartError)?
    {
        if field.name() == Some("iidm_file") {
            let bytes = field.bytes().await.map_err(UploadError::MultipartError)?;
            return serde_json::from_slice(&bytes).map_err(UploadError::JsonError);
        }
    }
    Err(UploadError::NoFile)
}

async fn update_ecs_state(state: &Arc<AppState>, network: &Network) {
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;

    let mut event_writer = world.resource_mut::<Events<RegisterEvent<Network>>>();
    event_writer.send(RegisterEvent {
        id: network.id(),
        component: network.clone(),
    });

    schedule.run(&mut world);
}
