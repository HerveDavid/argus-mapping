pub trait Identifiable {
    fn id(&self) -> String;
}

pub trait Updatable {
    type Updater;

    fn update(&mut self, updates: Self::Updater);
    fn update_from_json(&mut self, json: &str) -> Result<(), serde_json::Error>;
}
