//use rodio::{ OutputStream, Decoder, Sink };

use crate::prelude::*;

pub fn try_play_n_times(source: &Source, n: u16) -> Result<(), AudioError> {
    println!("playing sound of len {:?} {:?} times", source.bytes.len(), n);
    // still wont work
    /*
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for _ in 0..n {
        sink.append(
            Decoder::new(Cursor::new(source.clone()))
                .map_err(|_| AudioError::DecoderError)?
        );
    }
    sink.detach();
    */
    Ok(())
}

