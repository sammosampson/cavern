use crate::prelude::*;

pub const BOUNDS_MIN_Y: f32 = 32.0;
pub const BOUNDS_MAX_Y: f32 = SCREEN_HEIGHT - BOUNDS_MIN_Y;
pub const BAT_POSITION_OFFSET: f32 = 40.0;
pub const BAT_HEIGHT: f32 = 100.0;
pub const BAT_WIDTH: f32 = 18.0;
pub const HALF_BAT_HEIGHT: f32 = BAT_HEIGHT * 0.5;
pub const HALF_BAT_WIDTH: f32 = BAT_WIDTH * 0.5;
pub const BALL_DIAMETER: f32 = 14.0;
pub const BALL_RADIUS: f32 = BALL_DIAMETER* 0.5;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct WorldEntityId(String);

#[derive(Default, Debug, Copy, Clone)]
pub struct Ball;

#[derive(Default, Debug, Copy, Clone)]
pub struct Bat(pub u8);

impl Deref for Bat {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for WorldEntityId {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for WorldEntityId {
    fn from(name: String) -> Self {
        Self(name)
    }
}

pub struct Player;
pub struct Remove;

