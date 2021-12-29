use crate::prelude::*;

pub struct MenuScreen;

pub fn add_menu_screen(buffer: &mut CommandBuffer) {
    buffer.push((
        MenuScreen,
        Texture("Menu".to_string()), 
        Layer(3), 
        Position(centre_screen()), 
        WorldEntityId::from("Menu")
    ));
}

pub struct GameOverScreen;

pub fn add_game_over_screen(buffer: &mut CommandBuffer) {
    buffer.push((
        GameOverScreen,
        Texture("Over".to_string()), 
        Layer(3), 
        Position(centre_screen()), 
        WorldEntityId::from("GameOver")
    ));
}
