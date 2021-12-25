use crate::prelude::*;

#[system(for_each)]
pub fn exit_if_requested(
    event: &SystemEvent, 
    #[resource] game_state: &mut GameState,
) {
    match event {
        SystemEvent::CloseRequested => game_state.transition_to(GameStatus::Exiting),
        _ => {}
    }
}