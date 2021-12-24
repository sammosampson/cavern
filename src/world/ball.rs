use crate::prelude::*;

pub const BALL_DIAMETER: f32 = 14.0;
pub const BALL_RADIUS: f32 = BALL_DIAMETER* 0.5;

#[derive(Default, Debug, Copy, Clone)]
pub struct Ball;

pub fn add_ball(buffer: &mut CommandBuffer) {
    buffer.push((
        Ball,
        Texture(TextureResources::Ball), 
        Layer(1), 
        Position(centre_screen()), 
        MaximumVelocity(150.0),
        Heading(Angle::from_degrees(135.0).into()),
        WorldEntityId::from("Ball")
    ));
}
