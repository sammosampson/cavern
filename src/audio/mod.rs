mod player;
mod controller;
mod sound;
mod music;

pub use player::*;
pub use sound::*;
pub use music::*;
pub use controller::*;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawSoundSource {
    pub bytes: Vec<u8>,
}

impl AsRef<[u8]> for RawSoundSource {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

pub enum AudioEvent {
    PlaySound{ source: RawSoundSource, looped: bool, volume: f32 }
}

#[derive(Debug)]
pub enum AudioError {
    AudioFileReadError(FileError),
    PlayAudioError,
    SoundResourceNotFoundError,
    MusicResourceNotFoundError
}

impl From<FileError> for AudioError {
    fn from(error: FileError) -> Self {
        Self::AudioFileReadError(error)
    }
}

pub fn ogg(name: &str) -> String {
    let mut name = name.to_owned();
    name.push_str(".ogg");
    name
}