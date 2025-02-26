use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::{Path, State},
    response::{sse::Event, IntoResponse, Sse},
};
use futures::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::states::AppState;

pub async fn stream_iidm(
    Path((component_type, id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let ecs = state.ecs.read().await;
    let mut sse_registry = ecs.sse_registry.write().await;

    // Obtenir un canal d'abonnement pour ce composant
    let tx = { sse_registry.get_or_create_channel(&component_type, &id) };

    // S'abonner au canal
    let rx = tx.subscribe();

    // Créer un stream pour SSE
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(data) => {
                let event = Event::default().event("update").data(data);

                Some(Ok::<_, Infallible>(event))
            }
            Err(_) => {
                // Gérer la déconnexion ou l'erreur
                let event = Event::default().comment("keep-alive");
                Some(Ok::<_, Infallible>(event))
            }
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive"),
    )
}
