use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoundResources {
    WallImpact
}

#[derive(Debug)]
pub enum SoundError {
}

#[derive(Debug, Copy, Clone)]
pub struct Sound(SoundResources);

impl From<SoundResources> for Sound {
    fn from(from: SoundResources) -> Self {
        Self(from)
    }
}

impl Deref for Sound {
    type Target=SoundResources;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AudioPlayer;

impl AudioPlayer {
    pub fn new() -> Result<Self, SoundError>{
        Ok(Self)
    }

    
    pub fn play_sound(&self, sound: SoundResources) -> Result<(), SoundError> {
        println!("Playing sound: {:?}", sound);
        //TODO
        Ok(())
    }
}
