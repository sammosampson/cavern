mod resources;
use std::ops::Deref;

pub use resources::*;
use crate::prelude::*;

pub fn create_sound_components(resource: &str) -> (Sound, ) {
    (Sound(ogg(resource)), )
}

#[derive(Debug, Clone)]
pub struct Sound(String);

impl Deref for Sound {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}