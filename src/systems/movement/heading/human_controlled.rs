use crate::prelude::*;

#[system(for_each)]
#[read_component(Bat)]
#[write_component(Heading)]
pub fn set_player_one_heading_from_input(
    event: &SystemEvent,
    world: &mut SubWorld
) {
    set_player_heading_from_input(
        event,
        world, 
        PlayerIndex::Player1, 
        VirtualKeyCode::Q, 
        VirtualKeyCode::A
    );
}

#[system(for_each)]
#[read_component(Bat)]
#[write_component(Heading)]
pub fn set_player_two_heading_from_input(
    event: &SystemEvent,
    world: &mut SubWorld
) {
    set_player_heading_from_input(
        event,
        world, 
        PlayerIndex::Player2, 
        VirtualKeyCode::Up, 
        VirtualKeyCode::Down
    );
}

fn set_player_heading_from_input(
    event: &SystemEvent,
    world: &mut SubWorld,
    index: PlayerIndex,
    up_key: VirtualKeyCode,
    down_key: VirtualKeyCode
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            set_bat_heading(
                world,
                index, 
                get_heading(state, button, up_key, down_key));
        },
        _ => {}
    }  
}

fn set_bat_heading(world: &mut SubWorld, index: PlayerIndex, to_set: Vector) {
    let (heading, _) = <(&mut Heading, &Bat)>::query()
        .iter_mut(world)
        .filter(|(_, bat)| ***bat == index)
        .nth(0)
        .expect("No bat found");

    **heading = to_set;
}

fn get_heading(
    state: &InputState,
    button: &KeyboardButton,
    up_key: VirtualKeyCode,
    down_key: VirtualKeyCode
) -> Vector {
    if button.is_pressed(up_key, state) {
        return Vector::up(); 
    }
    else if button.is_pressed(down_key, state) {    
        return Vector::down(); 
    }
    Vector::default() 
}