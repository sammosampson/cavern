use crate::prelude::*;

pub fn create_wall_collision() -> WallCollision {
    WallCollision
}

pub fn create_bat_collision(bat: Entity, index: u8) -> BatCollision {
    BatCollision { index, bat }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallCollision;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatCollision {
    pub bat: Entity,
    pub index: u8
}
