use crate::prelude::*;

pub struct Menu;

pub fn add_menu(buffer: &mut CommandBuffer) {
    buffer.push((
        Menu,
        Texture(TextureResources::Menu(0)), 
        Layer(0), 
        Position(centre_screen()), 
        WorldEntityId::from("Menu")
    ));
}
