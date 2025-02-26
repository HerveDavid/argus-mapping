use std::sync::Arc;

use axum::extract::{Path, State};

use crate::states::AppState;

pub async fn stream_iidm(
    Path((component_type, id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) {
    todo!();
}
