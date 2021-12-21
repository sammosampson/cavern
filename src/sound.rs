use crate::prelude::*;
use std::io::{Cursor, BufReader};
use rodio::{OutputStream, OutputStreamHandle, Decoder, Sink};


#[derive(Debug, Copy, Clone)]
pub enum SoundResources {
    WallImpact
}

#[derive(Debug)]
pub enum SoundError {
    FailedToOpenStream,
    FailedToDecodeStream,
    FailedToCreateSink
}


#[derive(Debug, Copy, Clone)]
pub struct Sound(SoundResources);

impl Deref for Sound {
    type Target=SoundResources;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_wall_impact_sound() -> (WorldEntityId, Sound) {
    (WorldEntityId::from("wallImpactSound"), Sound(SoundResources::WallImpact))
}

pub struct AudioPlayer {
    stream_handle: OutputStreamHandle
}

impl AudioPlayer {
    pub fn new() -> Result<Self, SoundError>{
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|_| SoundError::FailedToOpenStream)?;
        
        let player = Self {
            stream_handle
        };

        Ok(player)
    }

    pub fn play_sound(&self, sound: SoundResources) -> Result<(), SoundError> {
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|_| SoundError::FailedToOpenStream)?;
        let cursor = Cursor::new(get_sound_resource(sound));
        let source = Decoder::new(BufReader::new(cursor)).map_err(|_| SoundError::FailedToDecodeStream)?;
        let sink = Sink::try_new(&stream_handle).map_err(|_| SoundError::FailedToCreateSink)?;
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }
}

fn get_sound_resource(sound: SoundResources) -> &'static[u8] {
    match sound {
        SoundResources::WallImpact => &include_bytes!("../sounds/hit_fast0.ogg")[..],
    }
}
