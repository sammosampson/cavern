
use crate::prelude::*;

#[system(simple)]
pub fn game_time(
    #[resource] game_timer: &mut GameTimer
) {
    game_timer.mark_frame();
}