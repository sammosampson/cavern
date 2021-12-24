pub fn create_wall_collision() -> WallCollision {
    WallCollision
}

pub fn create_bat_collision() -> BatCollision {
    BatCollision
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallCollision;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatCollision;
