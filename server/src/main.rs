use askama::Template;
use axum::{
    extract::Multipart,
    extract::State,
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

// Nouveau template pour le résultat JSON
#[derive(Template)]
#[template(path = "iidm_table.html")]
struct IIdmTableTemplate {
    iidm_table: String,
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

// Handler pour l'upload
async fn upload_iidm(mut multipart: Multipart) -> Html<String> {
    let iidm_table = String::from("Aucun fichier reçu");

    // while let Some(field) = multipart.next_field().await.unwrap() {
    //     if field.name() == Some("iidm_file") {
    //         if let Ok(bytes) = field.bytes().await {
    //             tracing::info!("{:?}", bytes);
    //             // if let Ok(json) = serde_json::from_slice::<Value>(&bytes) {
    //             //     json_content = serde_json::to_string_pretty(&json)
    //             //         .unwrap_or_else(|_| "Erreur de formatage JSON".to_string());
    //             // } else {
    //             //     json_content = "Fichier JSON invalide".to_string();
    //             // }
    //         }
    //     }
    // }

    let template = IIdmTableTemplate { iidm_table };
    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    // Initialisation du logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // État partagé
    let state = Arc::new(AppState {
        counter: AtomicU64::new(0),
    });

    // Construction du router
    let app = Router::new()
        .route("/", get(index))
        .route("/increment", post(increment))
        .route("/upload", post(upload_iidm))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Démarrage du serveur
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
