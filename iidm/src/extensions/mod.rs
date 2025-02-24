pub use iidm_derive::{Identifiable, Updatable};

use bevy_ecs::{schedule::Schedule, world::World};
use serde::{Deserialize, Serialize};

pub trait Identifiable {
    fn id(&self) -> String;
    fn register(&self, world: &mut World, schedule: &mut Schedule);
}

pub trait Updatable: Sized + Serialize + for<'de> Deserialize<'de> {
    type Updater: Send + Sync;
    type Err;

    fn update(&mut self, updates: Self::Updater);
    fn update_from_json(&mut self, json: &str) -> Result<(), Self::Err>;
}
