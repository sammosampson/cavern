mod rendering;
mod graph;

pub use rendering::*;
pub use graph::*;

pub enum EditorItems {
    EntityId = 10000,
    ExtractWindow,
    Position,
    Velocity,
    MaximumVelocity,
    CollisionBox
}

impl From<EditorItems> for EditorGraphDataItem {
    fn from(from: EditorItems) -> Self {
        (from as usize).into()
    }
}
