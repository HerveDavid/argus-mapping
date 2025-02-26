use askama::Template;
use axum::response::Html;
use iidm::Line;

#[derive(Template)]
#[template(path = "components/lines_section.html")]
pub struct LinesSectionTemplate {
    pub lines: Vec<Line>,
}

pub async fn lines_section_handler() -> Html<String> {
    let template = LinesSectionTemplate { lines: vec![] };

    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Error: {}", e)),
    )
}
