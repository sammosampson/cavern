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

#[system(for_each)]
pub fn editor_graph_entity_extraction(
    entity: &Entity,
    id: &WorldEntityId,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_string_entity_data(EditorItems::EntityId.into(), *entity, id.into());
}

#[system(for_each)]
pub fn editor_graph_position_extraction(
    entity: &Entity,
    position: &Position,
    next_position: &NextPosition,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_editable_vector_entity_data(EditorItems::Position.into(), *entity, **position);
    editor_graph.add_editable_vector_entity_data(EditorItems::NextPosition.into(), *entity, **next_position);
}

#[system(for_each)]
pub fn editor_graph_velocity_extraction(
    entity: &Entity,
    velocity: &Velocity,
    maximum_velocity: &MaximumVelocity,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_editable_vector_entity_data(EditorItems::Velocity.into(), *entity, **velocity);
    editor_graph.add_editable_float_entity_data(EditorItems::MaximumVelocity.into(), *entity, **maximum_velocity);
}