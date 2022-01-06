mod nodes;
mod events;

pub use nodes::*;
pub use events::*;

use crate::prelude::*;

pub fn create_editor_graph() -> EditorGraph {
    EditorGraph::new()
}

pub struct EditorGraph {
    controls: Vec<EditorGraphNode>, 
    state: EditorState,
    editor_visible: bool
}

impl EditorGraph {
    pub fn new() -> Self {
        Self {
            controls: vec!(),
            state: EditorState::default(),
            editor_visible: false
        }
    }
    
    pub fn editor_visible(&self) -> bool {
        self.editor_visible
    }

    pub fn toggle_editor_visibility(&mut self) {
        self.editor_visible = !self.editor_visible;
    }

    pub fn add_control(&mut self, control: EditorGraphNode) {
        self.controls.push(control);
    }

    pub fn controls(&self) -> &Vec<EditorGraphNode> {
        &self.controls
    }

    pub fn clear_data(&mut self) {
        //self.data = HashMap::default();
    }
}