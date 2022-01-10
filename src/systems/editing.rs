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
pub fn editor_event_handling(
    event: &SystemEvent,
    buffer: &mut CommandBuffer, 
    #[resource] editor_graph: &mut EditorGraph
) {
    match event { 
        SystemEvent::EditorChange(editor_event) => {
            match editor_event {
                EditorEvent::EntitySelected(entity, window_id) => {
                    editor_graph.windows_mut().select_entity(*entity, *window_id);
                },
                EditorEvent::VectorChanged(item, entity, value) => {
                    buffer.add_component(*entity, EditorVectorChange { item: *item, value: *value });
                },
                EditorEvent::FloatChanged(item, entity, value) => {
                    buffer.add_component(*entity, EditorFloatChange { item: *item, value: *value });
                }
            }
        },
        _ => {}
    }
}

#[system(for_each)]
#[filter(!component::<Added>())]
pub fn editor_graph_entity_extraction(
    entity: &Entity,
    id: &WorldEntityId,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_string_entity_data(EditorItems::EntityId.into(), *entity, id.into());
}

#[system(for_each)]
#[filter(component::<Remove>())]
pub fn editor_graph_entity_deletion(entity: &Entity, #[resource] editor_graph: &mut EditorGraph) {
    editor_graph.remove_string_entity_data(EditorItems::EntityId.into(), *entity);
}

#[system(for_each)]
pub fn editor_graph_position_extraction(
    entity: &Entity,
    position: &Position,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_vector_entity_data(EditorItems::Position.into(), *entity, **position);
}

#[system(for_each)]
pub fn editor_graph_velocity_extraction(
    entity: &Entity,
    velocity: &Velocity,
    maximum_velocity: &MaximumVelocity,
    #[resource] editor_graph: &mut EditorGraph
) {
    editor_graph.add_vector_entity_data(EditorItems::Velocity.into(), *entity, **velocity);
    editor_graph.add_float_entity_data(EditorItems::MaximumVelocity.into(), *entity, **maximum_velocity);
}