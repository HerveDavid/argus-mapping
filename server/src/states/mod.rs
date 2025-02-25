pub mod ecs;

use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub ecs: RwLock<ecs::EcsState>,
}
