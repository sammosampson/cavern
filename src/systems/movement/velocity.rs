use crate::prelude::*;

#[system(for_each)]
pub fn set_velocity_given_heading(
    maximum_velocity: &MaximumVelocity,
    velocity: &mut Velocity,
    heading: &Heading
) {
    let heading = **heading;
    let maximum_velocity = **maximum_velocity;

    if heading == Vector::default() {
        **velocity = 0.0;
    } else {
        **velocity = maximum_velocity;
    }
}

#[system(for_each)]
pub fn apply_velocity_to_position(
    velocity: &Velocity,
    heading: &Heading,
    position: &Position,
    next_position: &mut NextPosition,
    #[resource] game_timer: &GameTimer
) {
    let heading = **heading;
    let velocity = **velocity;
    let position = **position;

    **next_position = (heading * velocity * game_timer.last_frame_time()) + position
}