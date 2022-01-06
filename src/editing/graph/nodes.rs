use crate::prelude::*;

pub fn create_editor_sidebar(name: &str, children: Vec::<EditorGraphNode>) -> EditorGraphNode{
    EditorGraphNode::SideBar {
        name: name.to_string(),
        children
    }  
}

pub fn create_editor_window_toggle(title: &str, window_name: &'static str) -> EditorGraphNode {
    EditorGraphNode::Toggle {
        title: title.to_string(),
        click_handler: Box::new(move | visible | EditorEvent::SetWindowVisibility(visible, window_name.to_string()))
    }
}

pub enum EditorGraphNode {
    SideBar { name: String, children: Vec<EditorGraphNode> },
    Toggle { title: String, click_handler: Box<dyn Fn(bool) -> EditorEvent> },
}
