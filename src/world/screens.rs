use crate::prelude::*;

pub struct MenuScreenItem;

pub fn add_title_screen(buffer: &mut CommandBuffer) {
    buffer.push((
        MenuScreenItem,
        Texture::png("title"), 
        Layer(3), 
        Position(centre_screen()), 
        WorldEntityId::from("Title")
    ));
}

pub struct GameOverScreen;

pub fn add_game_over_screen(buffer: &mut CommandBuffer) {
    buffer.push((
        GameOverScreen,
        Texture::png("over"), 
        Layer(3), 
        Position(centre_screen()), 
        WorldEntityId::from("GameOver")
    ));
}
