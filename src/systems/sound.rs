use crate::prelude::*;

#[system(for_each)]
pub fn play_sound(
    entity: &Entity,
    sound: &Sound,
    #[resource] audio_player: &mut AudioPlayer,
    buffer: &mut CommandBuffer,
) {
    audio_player
        .play_sound(sound)
        .expect("sound could not be played");
        
    buffer.remove(*entity);
}   
