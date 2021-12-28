use crate::prelude::*;

#[system(for_each)]
pub fn play_sound(
    entity: &Entity,
    sound: &Sound,
    #[resource] source_cache: &mut SoundSourceCache,
    buffer: &mut CommandBuffer,
) {
    let sound_resource = **sound;
    println!("playing sound {:?}", sound_resource);
    
    let source = source_cache
        .get(&sound_resource)
        .expect("cannot find sound to play");

    try_play_n_times(source, 1)
        .expect("cannot play sound");
        
    buffer.remove(*entity);
}   
