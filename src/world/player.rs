use crate::prelude::*;

pub struct Player;

pub fn add_player(buffer: &mut CommandBuffer) {
    buffer.push((
        Texture::png("run13"), 
        Layer(3), 
        Position(Vector::new(HALF_SCREEN_WIDTH, 100.0)), 
        MaximumVelocity::from(1.0),
        WorldEntityId::from("Player"),
        Player,
        CollisionBox::from(Dimensions::from((64, 64)))
    ));
}