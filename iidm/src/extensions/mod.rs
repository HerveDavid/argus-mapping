pub use iidm_derive::{Identifiable, Updatable};
use serde::{Deserialize, Serialize};

pub trait Identifiable {
    fn id(&self) -> String;
}

pub trait Updatable: Sized + Serialize + for<'de> Deserialize<'de> {
    type Updater;
    type Err;

    fn update(&mut self, updates: Self::Updater);
    fn update_from_json(&mut self, json: &str) -> Result<(), Self::Err>;
}
