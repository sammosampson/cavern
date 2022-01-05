use crate::prelude::*;

#[system(for_each)]
pub fn set_heading_from_direction(direction: &Direction, heading: &mut Heading) {
    match direction {
        Direction::Left => **heading = Vector::left(),
        Direction::Right => **heading = Vector::right(),
    }
}

#[system(for_each)]
#[filter(!component::<Direction>())]
pub fn reset_heading_with_no_direction(heading: &mut Heading) {
    **heading = Vector::default();
}