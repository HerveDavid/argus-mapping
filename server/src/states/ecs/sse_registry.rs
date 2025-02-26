use std::collections::HashMap;
use tokio::sync::broadcast;

// Structure pour gérer les streams SSE par type de composant et ID
#[derive(Default)]
pub struct SseRegistry {
    // Map de (component_type, id) -> channel broadcast
    channels: HashMap<(String, String), broadcast::Sender<String>>,
}

impl SseRegistry {
    // Obtenir ou créer un canal pour un composant spécifique
    pub fn get_or_create_channel(
        &mut self,
        component_type: &str,
        id: &str,
    ) -> broadcast::Sender<String> {
        let c = format!("iidm::entities::{}", component_type);
        let key = (c, id.to_string());

        self.channels
            .entry(key)
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .clone()
    }

    // Publier une mise à jour pour un composant
    pub fn publish_update(&self, component_type: &str, id: &str, data: &str) {
        let key = (component_type.to_string(), id.to_string());

        match self.channels.get(&key) {
            Some(tx) => {
                let _ = tx.send(data.to_string());
                tracing::debug!("Published SSE update for {}/{}", component_type, id);
            }
            None => tracing::error!("Key not found: {}/{}", key.0, key.1,),
        }
    }
}
