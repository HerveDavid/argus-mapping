pub use iidm_derive::{Identifiable, Updatable};

pub trait Identifiable {
    fn id(&self) -> String;
}

pub trait Updatable: Sized {
    type Updater;
    type Err;

    fn update(&mut self, updates: Self::Updater);
    fn update_from_json(&mut self, json: &str) -> Result<(), Self::Err>;

    fn from_json_str(json: &str) -> Result<Self, Self::Err>;
    fn to_json_string(&self) -> Result<String, Self::Err>;
}
