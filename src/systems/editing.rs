use crate::prelude::*;

#[system(for_each)]
pub fn editor_visibility_from_input(
    event: &SystemEvent,
    #[resource] editor_graph: &mut EditorGraph
) {    
    match event {
        SystemEvent::KeyboardAction { state, button} => {
            if button.is_pressed(VirtualKeyCode::E, &state) {    
                editor_graph.toggle_editor_visibility();
            }
        },
        _ => {}
    }
}

#[system(for_each)]
pub fn editor_state_from_input(
    event: &SystemEvent,
    buffer: &mut CommandBuffer,
    #[resource] editor_graph: &mut EditorGraph
) {
    match event { 
        SystemEvent::EditorChange(editor_event) => {
            match editor_event {
                EditorEvent::SetWindowVisibility(item, visible, window_name) =>
                    editor_graph.set_window_visibility(*item, *visible, window_name.clone()),
                EditorEvent::VectorChanged(item, entity, vector) => {
                    buffer.add_component(*entity, EditorVectorChange { item: *item, vector: *vector });
                }
            }
        },
        _ => {}
    }
}

