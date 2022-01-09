mod rendering;
mod graph;
mod state;

pub use rendering::*;
pub use graph::*;
pub use state::*;

pub enum EditorItems {
    EntitiesWindowVisibility = 10000,
    EntityId,
    Position,
    NextPosition,
    Velocity,
    MaximumVelocity
}

impl From<EditorItems> for EditorGraphDataItem {
    fn from(from: EditorItems) -> Self {
        (from as usize).into()
    }
}
