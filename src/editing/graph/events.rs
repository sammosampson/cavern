use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    EntitySelected(Entity, u8),    
    ButtonClicked(EditorGraphDataItem, u8),    
    VectorChanged(EditorGraphDataItem, Entity, Vector),    
    DimensionsChanged(EditorGraphDataItem, Entity, Dimensions),    
    FloatChanged(EditorGraphDataItem, Entity, f32),    
}

pub struct EditorVectorChange {
    pub item: EditorGraphDataItem,
    pub value: Vector
}

pub struct EditorFloatChange {
    pub item: EditorGraphDataItem,
    pub value: f32
}