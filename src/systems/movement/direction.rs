use crate::prelude::*;

#[system(for_each)]
pub fn set_velocity_given_direction(
    maximum_velocity: &MaximumVelocity,
    velocity: &mut Velocity,
    direction: &Direction
) {
    match direction {
        Direction::Left => **velocity = Vector::new(-**maximum_velocity, velocity.y),
        Direction::Right => **velocity = Vector::new(**maximum_velocity, velocity.y),
    }
}

#[system(for_each)]
#[filter(!component::<Direction>())]
pub fn reset_velocity_with_no_direction(velocity: &mut Velocity) {
    **velocity = Vector::new(0.0, velocity.y)
}