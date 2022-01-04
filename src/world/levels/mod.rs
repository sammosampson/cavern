mod grid;
mod drawing;
mod background;

pub use grid::*;
pub use drawing::*;
pub use background::*;

use crate::prelude::*;

pub struct LevelNumber(usize);

impl Deref for LevelNumber {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for LevelNumber {
    fn from(from: usize) -> Self {
        Self(from)
    }
}