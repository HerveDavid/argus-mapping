mod handlers;
mod templates;

use axum::{
    routing::{get, get_service, post},
    Router,
};
use handlers::{increment, index, upload_iidm};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Share state
struct AppState {
    counter: AtomicU64,
}

#[tokio::main]
async fn main() {
    // Init log
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Init shared state
    let state = Arc::new(AppState {
        counter: AtomicU64::new(0),
    });

    // Build routes
    let app = Router::new()
        .route("/", get(index))
        .route("/increment", post(increment))
        .route("/upload", post(upload_iidm))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
