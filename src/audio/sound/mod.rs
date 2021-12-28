mod resources;
use std::ops::Deref;

pub use resources::*;

pub fn create_sound_components(resource: SoundResources) -> (Sound, ) {
    (Sound(resource), )
}


#[derive(Debug)]
pub enum AudioError {
}

#[derive(Debug, Copy, Clone)]
pub struct Sound(SoundResources);

impl Deref for Sound {
    type Target = SoundResources;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}