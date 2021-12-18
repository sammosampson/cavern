use crate::prelude::*;

pub struct Angle(pub f32);

impl Angle {
    pub fn from_degrees(angle: f32) -> Self {
        Self((angle / 180.0) * PI)
    }
}