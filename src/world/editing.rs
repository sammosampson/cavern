use crate::prelude::*;

const MAIN: &str = "Main";
const ENTITIES_WINDOW_NAME: &str = "Entities";
const ENTITIES_TOGGLE_TITLE: &str = ENTITIES_WINDOW_NAME;


pub fn add_editor_controls(editor_graph: &mut EditorGraph) {
    editor_graph.add_control(create_main_sidebar()); 
}

pub fn create_main_sidebar() -> EditorGraphNode {
    let entities_toggle = EditorGraphNode::Toggle {
        title: ENTITIES_TOGGLE_TITLE.to_string(),
        click_handler: Box::new(| visible | EditorEvent::SetWindowVisibility(visible, ENTITIES_WINDOW_NAME.to_string()))

    };

    EditorGraphNode::SideBar {
        name: MAIN.to_string(),
        children: vec!(entities_toggle)
    }
}