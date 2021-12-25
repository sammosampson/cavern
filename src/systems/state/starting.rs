use crate::prelude::*;

#[system(simple)]
pub fn transition_state_to_starting(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    _buffer: &mut CommandBuffer
) {
    if game_state.has_entered() {
        return;
    }

    game_state.enter(game_timer.total_game_time());
}