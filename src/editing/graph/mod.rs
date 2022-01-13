mod nodes;
mod events;
mod data;
mod windows;

pub use nodes::*;
pub use events::*;
pub use data::*;
pub use windows::*;

use crate::prelude::*;

pub fn create_editor_graph() -> EditorGraph {
    EditorGraph::new()
}

pub struct EditorGraph {
    windows: EditorGraphWindows,
    entity_data: HashMap<(Entity, EditorGraphDataItem), EditorGraphData>,
    editor_visible: bool
}

impl EditorGraph {
    pub fn new() -> Self {
        Self {
            windows: EditorGraphWindows::default(),
            entity_data: HashMap::default(),
            editor_visible: false
        }
    }
    
    pub fn editor_visible(&self) -> bool {
        self.editor_visible
    }

    pub fn toggle_editor_visibility(&mut self) {
        self.editor_visible = !self.editor_visible;
    }

    pub fn add_window(&mut self, controls: Vec<EditorGraphNode>) {
        self.windows.add(controls);
    }

    pub fn add_string_entity_data(&mut self, item: EditorGraphDataItem, entity: Entity, value: String) {
        self.add_entity_data(entity, item, EditorGraphData::EntityString { entity, value })
    }

    pub fn remove_string_entity_data(&mut self, item: EditorGraphDataItem, entity: Entity) {
        self.remove_entity_data(entity, item)
    }

    pub fn add_vector_entity_data(&mut self, item: EditorGraphDataItem, entity: Entity, value: Vector) {
        self.add_entity_data(entity, item, EditorGraphData::EntityVector { entity, value })
    }

    pub fn add_dimensions_entity_data(&mut self, item: EditorGraphDataItem, entity: Entity, value: Dimensions) {
        self.add_entity_data(entity, item, EditorGraphData::EntityDimensions { entity, value })
    }

    pub fn add_float_entity_data(&mut self, item: EditorGraphDataItem, entity: Entity, value: f32) {
        self.add_entity_data(entity, item, EditorGraphData::EntityFloat { entity, value })
    }

    pub fn add_entity_data(&mut self, entity: Entity, item: EditorGraphDataItem, value: EditorGraphData) {
        self.entity_data.insert((entity, item), value);
    }

    pub fn remove_entity_data(&mut self, entity: Entity, item: EditorGraphDataItem) {
        self.entity_data.remove(&(entity, item));
    }

    pub fn windows(&self) -> &EditorGraphWindows {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut EditorGraphWindows {
        &mut self.windows
    }

    pub fn entity_data(&self) -> &HashMap<(Entity, EditorGraphDataItem), EditorGraphData> {
        &self.entity_data
    }
}