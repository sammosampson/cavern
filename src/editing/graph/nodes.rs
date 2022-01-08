use crate::prelude::*;

pub fn create_editor_sidebar(name: &str, children: Vec::<EditorGraphNode>) -> EditorGraphNode{
    EditorGraphNode::SideBar {
        name: name.to_string(),
        children
    }  
}

pub fn create_editor_window(name: &str, children: Vec::<EditorGraphNode>) -> EditorGraphNode {
    EditorGraphNode::Window {
        name: name.to_string(),
        children
    }
}

pub fn create_editor_window_toggle(item: EditorGraphDataItem, title: &str, window_name: &'static str) -> EditorGraphNode {
    EditorGraphNode::Toggle {
        item,
        title: title.to_string(),
        click_handler: Box::new(move | visible | EditorEvent::SetWindowVisibility(item, visible, window_name.to_string())),
    }
}
pub fn create_editor_editable_vector(item: EditorGraphDataItem, title: &str, change_handler: Box<dyn Fn(Entity, Vector) -> EditorEvent>) -> EditorGraphNode {
    EditorGraphNode::Vector {
        item,
        title: title.to_string(),
        change_handler
    }
}

pub enum EditorGraphNode {
    SideBar { name: String, children: Vec<EditorGraphNode> },
    Window { name: String, children: Vec<EditorGraphNode> },
    Toggle { item: EditorGraphDataItem, title: String, click_handler: Box<dyn Fn(bool) -> EditorEvent> },
    Vector { item: EditorGraphDataItem, title: String, change_handler: Box<dyn Fn(Entity, Vector) -> EditorEvent> },
}
