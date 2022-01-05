use crate::prelude::*;

#[system(for_each)]
pub fn apply_velocity_to_position(
    velocity: &Velocity,
    position: &Position,
    next_position: &mut NextPosition,
    #[resource] game_timer: &GameTimer
) {
    let velocity = **velocity;
    let position = **position;

    **next_position = (velocity * PIXELS_PER_METER * game_timer.last_frame_time()) + position
}