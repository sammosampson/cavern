use crate::prelude::*;

pub const BASE_BALL_VELOCITY: f32 = 150.0;
pub const SLOW_BALL_VELOCITY: f32 = 300.0;
pub const MEDIUM_BALL_VELOCITY: f32 = 380.0;
pub const FAST_BALL_VELOCITY: f32 = 440.0;
pub const BALL_VELOCITY_INCREMENT: f32 = 10.0;
pub const BALL_DIAMETER: f32 = 14.0;
pub const BALL_RADIUS: f32 = BALL_DIAMETER* 0.5;

pub struct Ball;

pub fn add_ball(buffer: &mut CommandBuffer) {
    buffer.push((
        Ball,
        Texture(TextureResources::Ball), 
        Layer(2), 
        Position(centre_screen()), 
        MaximumVelocity(BASE_BALL_VELOCITY),
        Heading(Angle::from_degrees(135.0).into()),
        WorldEntityId::from("Ball")
    ));
}
