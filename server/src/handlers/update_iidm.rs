use std::sync::Arc;

use axum::{extract::State, Json};
use bevy_ecs::{event::Events, schedule};
use iidm::{Network, NetworkUpdate, UpdateEvent};
use serde::{Deserialize, Serialize};

use crate::states::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub id: String,
    pub component: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub status: String,
}

pub async fn update_iidm(State(state): State<Arc<AppState>>, Json(payload): Json<RegisterRequest>) {
}

async fn try_update_component(
    state: &Arc<AppState>,
    payload: &RegisterRequest,
) -> Result<(), String> {
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;

    if let Ok(update) = serde_json::from_value::<NetworkUpdate>(payload.component.clone()) {
        let id = payload.id.clone();
        let mut event_writer = world.resource_mut::<Events<UpdateEvent<Network>>>();
        event_writer.send(UpdateEvent { id, update });
        schedule.run(&mut world);
    }

    todo!()
}
