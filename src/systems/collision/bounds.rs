use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Bat>())]
pub fn contain_bat_in_bounds(
    position: &Position,
    heading: &mut Heading,
) {
    if position.y + HALF_BAT_HEIGHT > BOUNDS_MAX_Y && heading.y > 0.0 {
        **heading = Vector::default();
    }

    if position.y - HALF_BAT_HEIGHT < BOUNDS_MIN_Y && heading.y < 0.0 {
        **heading = Vector::default();
    }
}