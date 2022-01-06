use crate::prelude::*;

#[system(for_each)]
pub fn editor_visibility_from_input(
    event: &SystemEvent,
    #[resource] editor_graph: &mut EditorGraph
) {    
    match event {
        SystemEvent::KeyboardAction { state, button} => {
            if button.is_pressed(VirtualKeyCode::F11, &state) {    
                editor_graph.toggle_editor_visibility();
            }
        },
        _ => {}
    }
}