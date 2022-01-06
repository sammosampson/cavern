use crate::prelude::*;

pub enum EditorEvent {
    None,
    SetWindowVisibility(bool, String),    
}

pub enum EditorGraphNode {
    SideBar { name: String, children: Vec<EditorGraphNode> },
    Toggle { title: String, click_handler: Box<dyn Fn(bool) -> EditorEvent> },
}

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
    pub fn toggle_editor_visibility(&mut self) {
        self.editor_visible = !self.editor_visible;
    }

    pub fn add_control(&mut self, control: EditorGraphNode) {
        self.controls.push(control);
    }
}