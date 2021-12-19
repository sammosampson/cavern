use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn up() -> Self {
        Self {
            x: 0.0,
            y: 1.0
        }
    }

    pub fn down() -> Self {
        Self {
            x: 0.0,
            y: -1.0
        }
    }

    pub fn to_x_inverted(&self) -> Vector {
        Self::new(-self.x, self.y)
    }

    pub fn to_y_inverted(&self) -> Vector {
        Self::new(self.x, -self.y)
    }
}

impl Default for Vector {
    fn default() -> Self { 
        Self { x: 0.0, y: 0.0}
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl From<Dimensions> for Vector {
    fn from(from: Dimensions) -> Self {
        Self {
            x: from.width,
            y: from.height
        }
    }
}

impl From<Angle> for Vector {
    fn from(from: Angle) -> Self {
        Self {
            x: from.0.cos(),
            y: from.0.sin()
        }
    }
}