use crate::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct EditorState {
    pub windows_visible: HashMap<String, bool>
}
