use crate::prelude::*;

pub enum EditorGraphData {
    Boolean { value: bool },
    EntityString { entity: Entity, value: String },
    EntityFloat { entity: Entity, value: f32, editable: bool },   
    EntityVector { entity: Entity, value: Vector, editable: bool },   
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EditorGraphDataItem(usize);

impl From<usize> for EditorGraphDataItem {
    fn from(from: usize) -> Self {
        Self(from)
    }
}

impl Deref for EditorGraphDataItem {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}