use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tower_http::services::ServeDir;

// Définition du template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    count: u64,
}

#[derive(Template)]
#[template(path = "counter.html")]
struct CounterTemplate {
    count: u64,
}

// État partagé de l'application
struct AppState {
    counter: AtomicU64,
}

// Handler de la page d'accueil
async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = IndexTemplate {
        title: "Demo HTMX avec Rust".to_string(),
        count: state.counter.load(Ordering::Relaxed),
    };

    Html(template.render().unwrap())
}

// Handler pour l'incrémentation
async fn increment(State(state): State<Arc<AppState>>) -> Html<String> {
    let new_count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;

    let template = CounterTemplate { count: new_count };

    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    // Initialisation du logging
    tracing_subscriber::fmt::init();

    // État partagé
    let state = Arc::new(AppState {
        counter: AtomicU64::new(0),
    });

    // Construction du router
    let app = Router::new()
        .route("/", get(index))
        .route("/increment", post(increment))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .with_state(state);

    // Démarrage du serveur
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
