use crate::prelude::*;

#[system(for_each)]
pub fn apply_gravity_to_velocity(velocity: &mut Velocity) {
    velocity.y += GRAVITY;
}