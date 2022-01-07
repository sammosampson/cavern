use crate::prelude::*;

#[system(for_each)]
pub fn title_screen_input(
    event: &SystemEvent,
    #[resource] game_state: &mut GameState
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            if button.is_pressed(VirtualKeyCode::Space, &state) {    
                game_state.transition_to(GameStatus::Playing);
            }
        },
        _ => {}
    }  
}

#[system(for_each)]
pub fn play_screen_input(
    event: &SystemEvent,
    #[resource] game_state: &mut GameState
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            if button.is_pressed(VirtualKeyCode::Space, &state) {    
                game_state.transition_to(GameStatus::Finishing);
            }
        },
        _ => {}
    }  
}
