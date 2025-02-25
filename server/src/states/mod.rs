mod ecs;

use ecs::EcsState;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub ecs: RwLock<EcsState>,
}
