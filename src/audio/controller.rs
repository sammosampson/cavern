use rodio::{
    OutputStream,
    Decoder,
    Sink,
    Source
};

use std::sync::mpsc::Receiver;

use crate::prelude::*;

pub struct AudioController {
    receiver: Receiver<AudioEvent>,
}

impl AudioController {
    pub(crate) fn new(receiver: Receiver<AudioEvent>) -> Self {
        Self {
            receiver
        }
    }

    pub fn receive_audio_events(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        
        loop {
            if let Ok(message) = self.receiver.try_recv() {
                match message {
                    AudioEvent::PlaySound { source, looped, volume, } => {
                        let sink = Sink::try_new(&stream_handle)
                            .expect("Cannot create audio sink");

                        let source = Decoder::new(Cursor::new(source))
                            .expect("Cannot decode audio");
                        
                        if looped {
                            sink.append(source.repeat_infinite());
                        } else {
                            sink.append(source);
                        }
                        sink.set_volume(volume);
                        sink.detach();
                    }                
                }
            }
        }
    }
}