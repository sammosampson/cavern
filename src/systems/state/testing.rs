use crate::prelude::*;

#[system(simple)]
pub fn transition_state_to_testing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    #[resource] editor_graph: &mut EditorGraph,
    buffer: &mut CommandBuffer
) {
    if game_state.has_entered() {
        return;
    }
    
    add_editor_controls(editor_graph);
    add_player(buffer);

    
    game_state.enter(game_timer.total_game_time());
}