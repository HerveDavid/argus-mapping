use std::sync::Arc;

use askama::Template;
use axum::extract::{Multipart, State};
use axum::response::Html;
use bevy_ecs::event::Events;
use iidm::{Identifiable, Network, RegisterEvent};

use crate::states::AppState;

#[derive(Template)]
#[template(path = "iidm_table.html")]
struct IIdmTableTemplate {
    iidm_table: String,
    network: Option<Network>,
}

pub async fn upload_iidm(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Html<String> {
    let mut iidm_table = String::from("Aucun fichier reçu");

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("iidm_file") {
            if let Ok(bytes) = field.bytes().await {
                if let Ok(network) = serde_json::from_slice::<Network>(&bytes) {
                    iidm_table =
                        serde_json::to_string_pretty(&network).unwrap_or_else(|e| e.to_string());

                    // Load ecs mutables
                    let ecs = state.ecs.read().await;
                    let mut world = ecs.world.write().await;
                    let mut schedule = ecs.schedule.write().await;

                    // Get event writer
                    let mut event_writer = world.resource_mut::<Events<RegisterEvent<Network>>>();
                    event_writer.send(RegisterEvent {
                        id: network.id(),
                        component: network,
                    });

                    // Apply state changed
                    schedule.run(&mut world);
                } else {
                    iidm_table = "Invalid JSON file".to_string();
                }
            }
        }
    }

    let network: Option<Network> = serde_json::from_str(&iidm_table).ok();
    let template = IIdmTableTemplate {
        iidm_table: iidm_table.clone(),
        network,
    };
    Html(template.render().unwrap())
}
