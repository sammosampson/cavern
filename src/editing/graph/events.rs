use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    SetWindowVisibility(EditorGraphDataItem, bool, String),    
    VectorChanged(EditorGraphDataItem, Entity, Vector),    
}

pub struct EditorVectorChange {
    pub item: EditorGraphDataItem,
    pub vector: Vector
}