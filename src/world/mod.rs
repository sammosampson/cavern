mod player;
mod ball;
mod arena;
mod impact;
mod menu;

pub use player::*;
pub use ball::*;
pub use arena::*;
pub use impact::*;
pub use menu::*;

use crate::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct WorldEntityId(String);

impl core::fmt::Debug for WorldEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for WorldEntityId {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for WorldEntityId {
    fn from(name: String) -> Self {
        Self(name)
    }
}
pub struct Remove;

pub fn remove_entity(buffer: &mut CommandBuffer, entity: Entity) {
    buffer.add_component(entity, Remove)
}

pub fn centre_screen() -> Vector {
    Vector::new(HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT)
}

