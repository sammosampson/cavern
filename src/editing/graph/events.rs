use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    SetWindowVisibility(EditorGraphDataItem, bool, String),    
}
