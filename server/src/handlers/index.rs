use askama::Template;
use axum::extract::State;
use axum::response::Html;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::AppState;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    pub title: String,
    pub count: u64,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = IndexTemplate {
        title: "Demo HTMX avec Rust".to_string(),
        count: state.counter.load(Ordering::Relaxed),
    };

    Html(template.render().unwrap())
}
