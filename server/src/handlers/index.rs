use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> Html<String> {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}
