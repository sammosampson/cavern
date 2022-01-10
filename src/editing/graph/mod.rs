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

#[derive(Copy, Clone)]
pub enum EditorGraphWindowStyle {
    SideBar,
    Window,
}

#[derive(Default)]
pub struct EditorGraphWindows {
    windows_cursor: u8,
    windows: HashMap<u8, EditorGraphWindow>
}

impl EditorGraphWindows {
    fn add(&mut self, controls: Vec<EditorGraphNode>) {
        let id = self.get_next_window_id();
        let style = self.get_next_style();
        self.windows.insert(id,EditorGraphWindow::new(id, style, controls));
    }

    fn get_next_window_id(&mut self) -> u8 {
        self.windows_cursor += 1;
        self.windows_cursor
    }

    fn get_next_style(&self) -> EditorGraphWindowStyle {
        if self.windows.is_empty() {
            EditorGraphWindowStyle::SideBar
        } else {
            EditorGraphWindowStyle::Window
        }
    }

    pub fn select_entity(&mut self, entity: Entity, window_id: u8) {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.select_entity(entity);
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<u8, EditorGraphWindow>{
        self.windows.values()
    }
}

pub struct EditorGraphWindow {
    id: u8,
    controls: Vec<EditorGraphNode>,
    style: EditorGraphWindowStyle,
    selected_entity: Option<Entity>
}

impl EditorGraphWindow {
    fn new(id: u8, style: EditorGraphWindowStyle, controls: Vec<EditorGraphNode>) -> Self {
        Self {
            id,
            controls,
            style,
            selected_entity: None
        }
    }

    pub fn select_entity(&mut self, entity: Entity) {
        self.selected_entity = Some(entity);
    }

    pub fn selected_entity(&self) -> Option<Entity> {
        self.selected_entity
    }

    pub fn is_selected_entity(&self, entity: &Entity) -> bool {
        if let Some(selected_entity)  = self.selected_entity {
            return selected_entity == *entity;
        }
        false
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn style(&self) -> EditorGraphWindowStyle {
        self.style
    }

    pub fn controls(&self) -> &Vec<EditorGraphNode> {
        &self.controls
    }
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
            editor_visible: true
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