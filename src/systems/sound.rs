use crate::prelude::*;

#[system(for_each)]
pub fn play_sound(
    entity: &Entity,
    sound: &Sound,
    #[resource] audio_player: &mut AudioPlayer,
    buffer: &mut CommandBuffer,
) {
    audio_player.play_sound(**sound).unwrap();
    buffer.remove(*entity);
}   
