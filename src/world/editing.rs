use crate::prelude::*;

pub fn add_editor_controls(editor_graph: &mut EditorGraph) {
    editor_graph.add_window(create_movement_tab()); 
}

fn create_movement_tab() -> Vec<EditorGraphNode> {
    vec!(
        create_editor_entities_list(),
        create_editor_position(),
        create_editor_velocity(),
        create_editor_maximum_velocity(),
    )
}