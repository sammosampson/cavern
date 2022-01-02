use std::sync::mpsc::{
    Sender,
    channel
};

use crate::prelude::*;

pub fn create_audio_player() -> AudioPlayer {
    AudioPlayer::new()
}

pub struct AudioPlayer {
    event_sender: Sender<AudioEvent>
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (event_sender, receiver) = channel();

        std::thread::spawn(move || {
            let controller = AudioController::new(receiver);
            controller.receive_audio_events();
        });

        AudioPlayer { event_sender }
    }

    pub fn play_sound(&mut self, resources: &SoundSourceCache, resource: &str, volume: f32) -> Result<(), AudioError> {
        let source = resources.get(resource).ok_or(AudioError::SoundResourceNotFoundError)?;
        println!("playing sound {:?}", resource);
        self.play_audio(source, volume, false)
    }

    pub fn play_music(&mut self, resources: &MusicSourceCache, resource: &str, volume: f32) -> Result<(), AudioError> {
        let source = resources.get(resource).ok_or(AudioError::MusicResourceNotFoundError)?;
        println!("playing music {:?}", resource);
        self.play_audio(source, volume, true)
    }

    fn play_audio(&mut self, source: &RawSoundSource, volume: f32, looped: bool) -> Result<(), AudioError> {
        Ok(self.event_sender
           .send(AudioEvent::PlaySound { source: source.clone(), looped, volume })
           .map_err(|_| AudioError::PlayAudioError)?)
    }
}

