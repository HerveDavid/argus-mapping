use askama::Template;
use axum::extract::State;
use axum::response::Html;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::AppState;

#[derive(Template)]
#[template(path = "counter.html")]
struct CounterTemplate {
    count: u64,
}

pub async fn increment(State(state): State<Arc<AppState>>) -> Html<String> {
    let new_count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;

    let template = CounterTemplate { count: new_count };

    Html(template.render().unwrap())
}
