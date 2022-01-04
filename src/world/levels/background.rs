use crate::prelude::*;
 
pub fn add_level_background(buffer: &mut CommandBuffer) {
    buffer.push((
        Texture::png("bg0"), 
        Layer(0), 
        Position(centre_screen()), 
        WorldEntityId::from("Level")
    ));
}