use crate::prelude::*;

#[system(simple)]
pub fn transition_state_to_finishing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer
) {
    if game_state.has_entered() {
        return;
    }

    add_game_over_screen(buffer);
    
    game_state.enter(game_timer.total_game_time());
}
