mod handlers;
mod states;

use axum::{
    routing::{get, get_service, post},
    Router,
};
use handlers::{index, update_iidm, upload_iidm};
use states::AppState;
use std::{path::PathBuf, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Init log
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Path for js dependencies
    let static_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    // Build routes
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload_iidm))
        .route("/update/{component_type}", post(update_iidm))
        .nest_service("/static", get_service(ServeDir::new(static_path)))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(AppState::default()));

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("Server started http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
