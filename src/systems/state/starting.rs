use crate::prelude::*;

#[system(simple)]
pub fn transition_state_to_starting(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer
) {
    if game_state.has_entered() {
        return;
    }
    
    add_level_background(buffer);
    add_title_screen(buffer);
    add_press_space_title(buffer, game_timer);
    
    game_state.enter(game_timer.total_game_time());
}