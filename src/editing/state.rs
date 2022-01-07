use crate::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct EditorState {
    pub windows_visible: HashMap<String, bool>
}

impl EditorState {
    pub fn set_window_visibility(&mut self, visible: bool, window_name: String) {
        self.windows_visible.insert(window_name, visible);

    }

    pub fn is_window_visible(&self, window_name: &str) -> bool {
        self.windows_visible.contains_key(window_name) 
            && *self.windows_visible.get(window_name).unwrap()
    }
}
