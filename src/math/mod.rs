mod matrices;
mod vector;

pub use std::f64::consts;
pub use matrices::*;
pub use vector::*;

use crate::prelude::*;

//pub const PI:f32 = consts::PI as f32;

#[derive(Copy, Clone, Debug)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32
}

impl From<(u32, u32)> for Dimensions {
    fn from(from: (u32, u32)) -> Self {
        Dimensions {
            width: from.0 as f32,
            height: from.1 as f32,
        }
    }
}

impl Mul<f32> for Dimensions {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}