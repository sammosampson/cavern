use crate::prelude::*;

const MAIN: &str = "Main";
const ENTITIES_WINDOW_NAME: &str = "Entities";

pub enum EditorItems {
    EntitiesWindowVisibility
}

impl From<EditorItems> for EditorGraphDataItem {
    fn from(from: EditorItems) -> Self {
        (from as usize).into()
    }
}

pub fn add_editor_controls(editor_graph: &mut EditorGraph) {
    editor_graph.add_control(create_main_sidebar()); 
    editor_graph.add_control(create_entities_window()); 
}

fn create_main_sidebar() -> EditorGraphNode {
    create_main_sidebar_with_children(
        vec!(create_entities_window_visibility_toggle())
    )
}

fn create_entities_window() -> EditorGraphNode {
    create_editor_window(
        ENTITIES_WINDOW_NAME, 
        vec!()
    )
}

fn create_main_sidebar_with_children(children: Vec<EditorGraphNode>) -> EditorGraphNode {
    create_editor_sidebar(MAIN, children)
}

fn create_entities_window_visibility_toggle() -> EditorGraphNode {
    create_editor_window_toggle(
        EditorItems::EntitiesWindowVisibility.into(), 
        "Entities", 
        ENTITIES_WINDOW_NAME
    )
}