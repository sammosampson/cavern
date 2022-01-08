mod nodes;
mod events;
mod data;

pub use nodes::*;
pub use events::*;
pub use data::*;

use crate::prelude::*;

pub fn create_editor_graph() -> EditorGraph {
    EditorGraph::new()
}

pub struct EditorGraph {
    controls: Vec<EditorGraphNode>, 
    data: HashMap<EditorGraphDataItem, EditorGraphData>,
    state: EditorState,
    editor_visible: bool
}

impl EditorGraph {
    pub fn new() -> Self {
        Self {
            controls: vec!(),
            data: HashMap::default(),
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

    pub fn add_boolean_data(&mut self, item: EditorGraphDataItem, value: bool) {
        self.add_data(item, EditorGraphData::Boolean { value })
    }

    pub fn add_editable_vector_data(&mut self, item: EditorGraphDataItem, entity: Entity, value: Vector) {
        self.add_data(item, EditorGraphData::EditableVector { entity, value })
    }
    
    pub fn add_data(&mut self, item: EditorGraphDataItem, value: EditorGraphData) {
        self.data.insert(item, value);
    }

    pub fn controls(&self) -> &Vec<EditorGraphNode> {
        &self.controls
    }

    pub fn clear_data(&mut self) {
        self.data = HashMap::default();
    }

    pub fn data(&self) -> &HashMap<EditorGraphDataItem, EditorGraphData> {
        &self.data
    }

    pub fn set_window_visibility(&mut self, item: EditorGraphDataItem, visible: bool, window_name: String) {
        self.add_boolean_data(item, visible);
        self.state.set_window_visibility(visible, window_name);

    }

    pub fn is_window_visible(&self, window_name: &str) -> bool {
        self.state.is_window_visible(window_name)
    }
}