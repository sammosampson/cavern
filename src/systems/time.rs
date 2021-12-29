
use crate::prelude::*;

#[system(simple)]
pub fn calculate_elapsed_time(
    #[resource] game_timer: &mut GameTimer
) {
    game_timer.mark_frame();
}