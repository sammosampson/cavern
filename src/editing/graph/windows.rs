

use crate::prelude::*;

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
    pub fn add(&mut self, controls: Vec<EditorGraphNode>) {
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

    pub fn copy_window(&mut self, window_id: u8) {
        if !self.windows.contains_key(&window_id) {
            return;
        }
        
        let controls = self.windows
            .get(&window_id)
            .unwrap()
            .controls()
            .to_vec();

        self.add(controls);

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