use crate::prelude::*;

#[system(for_each)]
pub fn set_position(
    position: &mut Position,
    next_position: &NextPosition
) {
    **position = **next_position
}