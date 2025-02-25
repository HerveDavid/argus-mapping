use crate::states::AppState;
use askama::Template;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use iidm::*;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("Multipart field error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] NetworkError),
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
    let result = process_upload(&mut multipart).await;

    match result {
        Ok(network) => {
            update_ecs_state(&state, &network).await;
            let template = IIdmTableTemplate::new("".to_string(), Some(network));
            let html = template.render().map_err(UploadError::TemplateError)?;
            Ok(Html(html))
        }
        Err(err) => {
            let error_message = err.to_string();
            let template = IIdmTableTemplate::new(error_message, None);
            let html = template.render().map_err(UploadError::TemplateError)?;
            Ok(Html(html))
        }
    }
}

async fn process_upload(multipart: &mut Multipart) -> Result<Network, UploadError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(UploadError::MultipartError)?
    {
        if field.name() == Some("iidm_file") {
            let bytes = field.bytes().await.map_err(UploadError::MultipartError)?;
            return serde_json::from_slice(&bytes)
                .map_err(|e| NetworkError::Deserialization(e))
                .map_err(UploadError::JsonError);
        }
    }
    Err(UploadError::NoFile)
}

async fn update_ecs_state(state: &Arc<AppState>, network: &Network) {
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;

    network.register(&mut world, &mut schedule);
}
