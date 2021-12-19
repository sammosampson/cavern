use crate::prelude::*;

use std::{io::BufReader, fs::File, sync::Arc};
use rodio::{Decoder, OutputStream, source::Source, buffer::SamplesBuffer};

pub fn create_wall_impact_sound() -> (WorldEntityId, Sound) {
    play_sound(SoundResources::WallImpact);
    (WorldEntityId::from("wallImpactSound"), Sound(SoundResources::WallImpact))
}

pub enum SoundResources {
    WallImpact
}

pub struct Sound(SoundResources);

pub fn get_sound_resource(sound: SoundResources) -> &'static[u8] {
    match sound {
        SoundResources::WallImpact => &include_bytes!("../sounds/bounce0.ogg")[..],
    }
}

pub fn play_sound(sound: SoundResources) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //get_sound_resource(sound)
    //let file = BufReader::new(File::open("sounds/bounce0.ogg").unwrap());
    let file = RawSound::load(sound).unwrap();
    stream_handle.play_raw(file.decoder().convert_samples()).unwrap();
}

pub struct RawSound (Arc<Vec<u8>>);

impl AsRef<[u8]> for RawSound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl RawSound {
    pub fn load(sound: SoundResources) -> std::io::Result<RawSound> {
        Ok(RawSound(Arc::new(get_sound_resource(sound).to_vec())))
    }
    pub fn cursor(self: &Self) -> std::io::Cursor<RawSound> {
        std::io::Cursor::new(RawSound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<std::io::Cursor<RawSound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}
