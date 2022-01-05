mod matrices;
mod vector;

pub use std::f64::consts;
pub use matrices::*;
pub use vector::*;

use crate::prelude::*;

//pub const PI:f32 = consts::PI as f32;
pub const INFINITY: f32 = f32::INFINITY;

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

impl From<f32> for Dimensions {
    fn from(from: f32) -> Self {
        Self { width: from, height: from }
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

pub fn min(left: f32, right: f32) -> f32 {
    if left < right {
        return left;
    }
    right
}

pub fn max(left: f32, right: f32) -> f32 {
    if left > right {
        return left;
    }
    right
}