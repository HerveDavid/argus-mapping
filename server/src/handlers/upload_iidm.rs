use askama::Template;
use axum::extract::Multipart;
use axum::response::Html;

#[derive(Template)]
#[template(path = "iidm_table.html")]
struct IIdmTableTemplate {
    iidm_table: String,
}

pub async fn upload_iidm(mut multipart: Multipart) -> Html<String> {
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
