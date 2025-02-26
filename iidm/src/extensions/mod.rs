pub use iidm_derive::{Identifiable, Updatable};

use bevy_ecs::{schedule::Schedule, world::World};
use serde::{Deserialize, Serialize};

pub trait Identifiable {
    fn id(&self) -> String;
    fn register(&self, world: &mut World, schedule: &mut Schedule);
}

pub trait Updatable: Sized + Serialize + for<'de> Deserialize<'de> {
    type Updater: Send + Sync;

    fn update(&mut self, updates: Self::Updater);
}

pub trait JsonSchema: for<'de> Deserialize<'de> + Serialize {
    type Err;
    fn fields_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, Self::Err>;
}
