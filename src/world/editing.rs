use crate::prelude::*;

const MAIN: &str = "Main";
const ENTITIES_WINDOW_NAME: &str = "Entities";
const ENTITIES_TOGGLE_TITLE: &str = ENTITIES_WINDOW_NAME;


pub fn add_editor_controls(editor_graph: &mut EditorGraph) {
    editor_graph.add_control(create_main_sidebar()); 
}

fn create_main_sidebar() -> EditorGraphNode {
    create_editor_sidebar(
        MAIN, 
        vec!(create_editor_window_toggle(ENTITIES_TOGGLE_TITLE, ENTITIES_WINDOW_NAME))
    )
}