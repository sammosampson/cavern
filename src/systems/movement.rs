use crate::prelude::*;

#[system(for_each)]
#[filter(component::<MaximumVelocity>())]
#[filter(!component::<Velocity>())]
#[filter(!component::<NextPosition>())]
pub fn initialise_movement(
    entity: &Entity,
    position: &Position,
    buffer: &mut CommandBuffer
) {
    buffer.add_component(*entity, NextPosition(position.0));
    buffer.add_component(*entity, Velocity::default());
}

#[system(for_each)]
#[read_component(Bat)]
#[read_component(Heading)]
#[write_component(Heading)]
pub fn set_heading_from_input(
    event: &SystemEvent,
    world: &mut SubWorld
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            let mut query = <(&mut Heading, &Bat)>::query().filter(component::<Player>());
            for (heading, bat) in query.iter_mut(world) {
                let up_key = if **bat == PlayerIndex::Player1 { VirtualKeyCode::Q } else  { VirtualKeyCode::Up };
                let down_key = if **bat == PlayerIndex::Player1 { VirtualKeyCode::A } else  { VirtualKeyCode::Down };
                if button.is_pressed(up_key, &state) {    
                    heading.0 = Vector::up(); 
                }
                else if button.is_pressed(down_key, &state) {    
                    heading.0 = Vector::down(); 
                } else {
                    heading.0 = Vector::default(); 
                }
            }
        },
        _ => {}
    }  
}

#[system(for_each)]
pub fn set_velocity_given_heading(
    maximum_velocity: &MaximumVelocity,
    velocity: &mut Velocity,
    heading: &Heading
) {
    if heading.0 == Vector::default() {
        velocity.0 = 0.0;
    } else {
        velocity.0 = maximum_velocity.0;  
    }
}

#[system(for_each)]
pub fn apply_velocity_to_position(
    velocity: &Velocity,
    heading: &Heading,
    position: &Position,
    next_position: &mut NextPosition,
    #[resource] game_timer: &GameTimer
) {
    next_position.0 = (heading.0 * velocity.0 * game_timer.last_frame_time()) + position.0
}

#[system(for_each)]
pub fn movement(
    entity_id: &WorldEntityId,
    position: &mut Position,
    next_position: &NextPosition,
    #[resource] item_renderer: &mut ItemRenderer
) {
    if let Some(item) = item_renderer.find_mut(entity_id) {
        item.set_centre_position(position.0);
        position.0 = next_position.0
    }
}
