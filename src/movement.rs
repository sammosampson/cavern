use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right
}

#[derive(Debug, Copy, Clone)]
pub struct Position(pub Vector);

impl Deref for Position {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl DerefMut for NextPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
