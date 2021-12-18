use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Position(pub Vector);

impl Deref for Position {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NextPosition(pub Vector);

impl Deref for NextPosition {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Velocity(pub f32);

#[derive(Debug, Copy, Clone, Default)]
pub struct MaximumVelocity(pub f32);

#[derive(Default, Debug, Copy, Clone)]
pub struct Heading(pub Vector);


