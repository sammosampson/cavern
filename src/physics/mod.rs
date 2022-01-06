mod collisions;
mod velocity;

pub use collisions::*;
pub use velocity::*;

pub const GRAVITY:f32 = 0.01;
pub const PIXELS_PER_METER: f32 = 64.0;