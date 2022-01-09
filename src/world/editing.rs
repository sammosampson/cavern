use crate::prelude::*;

const MAIN: &str = "Main";
const ENTITIES_WINDOW_NAME: &str = "Entities";

pub enum EditorItems {
    EntitiesWindowVisibility,
    Position
}

impl From<EditorItems> for EditorGraphDataItem {
    fn from(from: EditorItems) -> Self {
        (from as usize).into()
    }
}

pub fn add_editor_controls(editor_graph: &mut EditorGraph) {
    editor_graph.add_control(create_main_sidebar()); 
}

fn create_main_sidebar() -> EditorGraphNode {
    create_main_sidebar_with_children(vec!(
        create_entities_window_visibility_toggle(),
        create_position_vector()
    ))
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

fn create_position_vector() -> EditorGraphNode {
    let item = EditorItems::Position.into();
    create_editor_editable_vector(
        item, 
        "Position", 
        Box::new(move | entity, position| EditorEvent::VectorChanged(item, entity, position))
    )
}