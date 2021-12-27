use crate::prelude::*;

#[system(simple)]
#[read_component(Bat)]
pub fn transition_state_to_finishing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {
    if game_state.has_entered() {
        return;
    }

    super::set_normal_bat_textures(buffer, world);
    add_game_over_screen(buffer);
    
    game_state.enter(game_timer.total_game_time());
}
