mod ecs;

use std::sync::atomic::AtomicU64;

use ecs::EcsState;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub counter: AtomicU64,
    pub ecs: RwLock<EcsState>,
}
