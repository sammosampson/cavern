use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Remove>())]
pub fn remove_entity(
    entity: &Entity,
    id: &WorldEntityId,
    buffer: &mut CommandBuffer,
) {
    println!("remove entity {:?}", id);
    buffer.remove(*entity);
}   

#[system(for_each)]
#[read_component(Player)]
#[write_component(Direction)]
pub fn set_player_direction_from_input(
    event: &SystemEvent,
    world: &mut SubWorld,
    buffer: &mut CommandBuffer
) {
    match event {
        SystemEvent::KeyboardAction { state, button } => {
            if button.is_pressed(VirtualKeyCode::Left, state) {
                set_direction(buffer, world, Direction::Left)
            } else if button.is_pressed(VirtualKeyCode::Right, state) {
                set_direction(buffer, world, Direction::Right)
            } else {
                remove_direction(buffer, world)
            }
        },
        _ => {}
    }  
}

fn set_direction(buffer: &mut CommandBuffer, world: &mut SubWorld, direction: Direction) {
    buffer.add_component(get_player(world), direction);

}

fn remove_direction(buffer: &mut CommandBuffer, world: &mut SubWorld) {
    buffer.remove_component::<Direction>(get_player(world));
}

fn get_player(world: &SubWorld) -> Entity {
    <Entity>::query()
        .filter(component::<Player>())
        .iter(world)
        .map(|entity| *entity)
        .nth(0)
        .expect("No player found")
}
