
use crate::prelude::*;

#[system(simple)]
#[read_component(ElapsedTime)]
#[write_component(ElapsedTime)]
pub fn calculate_elapsed_time(
    #[resource] game_timer: &mut GameTimer,
    world: &mut SubWorld
) {
    game_timer.mark_frame();
    let elapsed_time = game_timer.last_frame_time();

    let mut query = <&mut ElapsedTime>::query();

    for time in query.iter_mut(world) {
        time.0 = elapsed_time;
    }
}