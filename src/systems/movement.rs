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
pub fn set_bat_movement_sounds(
    heading: &Heading,
    buffer: &mut CommandBuffer
) {
    let heading = **heading;

    if heading == Vector::down() {
        add_bat_up_sounds(buffer)
    } else if heading == Vector::up() {
        add_bat_down_sounds(buffer)
    }
}


#[system(for_each)]
pub fn set_velocity_given_heading(
    maximum_velocity: &MaximumVelocity,
    velocity: &mut Velocity,
    heading: &Heading
) {
    let heading = **heading;
    let maximum_velocity = **maximum_velocity;

    if heading == Vector::default() {
        **velocity = 0.0;
    } else {
        **velocity = maximum_velocity;
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
    let heading = **heading;
    let velocity = **velocity;
    let position = **position;

    **next_position = (heading * velocity * game_timer.last_frame_time()) + position
}

#[system(for_each)]
pub fn movement(
    entity_id: &WorldEntityId,
    position: &mut Position,
    next_position: &NextPosition,
    #[resource] item_renderer: &mut ItemRenderer
) {
    if let Some(item) = item_renderer.find_mut(entity_id) {
        item.set_centre_position(**position);
        **position = **next_position
    }
}
