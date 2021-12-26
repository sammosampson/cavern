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

    add_menu_screen(buffer);
    add_arena(buffer);
    add_bat0(buffer);
    add_bat1(buffer);

    game_state.enter(game_timer.total_game_time());
}