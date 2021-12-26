use crate::prelude::*;

#[system(for_each)]
pub fn set_game_style_from_input(
    event: &SystemEvent,
    #[resource] game_style: &mut GameStyle,
    #[resource] game_state: &mut GameState
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            if button.is_pressed(VirtualKeyCode::Down, &state) {    
                *game_style = GameStyle::TwoPlayer;
            }
            if button.is_pressed(VirtualKeyCode::Up, &state) {    
                *game_style = GameStyle::OnePlayer;
            }
            if button.is_pressed(VirtualKeyCode::Space, &state) {    
                game_state.transition_to(GameStatus::Playing);
            }
        },
        _ => {}
    }  
}

#[system(for_each)]
#[filter(component::<Menu>())]
pub fn set_game_style_texture(
    entity: &Entity,
    #[resource] game_style: &mut GameStyle,
    buffer: &mut CommandBuffer
) {
    match game_style { 
        GameStyle::OnePlayer => {
            set_texture(buffer, *entity, TextureResources::Menu(0));
        }
        GameStyle::TwoPlayer => {
            set_texture(buffer, *entity, TextureResources::Menu(1));
        }
    }  
}