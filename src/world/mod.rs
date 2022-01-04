mod player;
mod levels;
mod titles;
mod screens;

pub use screens::*;
pub use levels::*;
pub use titles::*;
pub use player::*;

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

pub fn play_music(buffer: &mut CommandBuffer) {
    buffer.push(create_music_components("theme"));   
}

pub fn add_player_die_sound(buffer: &mut CommandBuffer) {
    buffer.push(create_sound_components("die0"));   
}