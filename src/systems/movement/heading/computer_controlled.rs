use crate::prelude::*;

#[system(simple)]
#[read_component(Bat)]
#[read_component(Ball)]
#[read_component(Position)]
#[write_component(Heading)]
pub fn calculate_computer_player_two_heading(
    world: &SubWorld,
    buffer: &mut CommandBuffer
) {
    let (bat_entity, bat_position) = get_bat(world);
    let ball_position = get_ball_position(world);
    
    if ball_position.y < bat_position.y {
        buffer.add_component(bat_entity, Heading(Vector::down()))
    } else if ball_position.y > bat_position.y {
        buffer.add_component(bat_entity, Heading(Vector::up()))
    } else {
        buffer.add_component(bat_entity, Heading(Vector::default()))
    }
}

fn get_bat(world: &SubWorld) -> (Entity, Vector) {
    <(Entity, &Position, &Bat)>::query()
        .iter(world)
        .filter(|(_, __, bat)| ***bat == PlayerIndex::Player2)
        .map(|(entity, position, _)| (*entity, **position))
        .nth(0)
        .expect("No bat found")
        
}

fn get_ball_position(world: &SubWorld) -> Vector {
    <&Position>::query()
        .filter(component::<Ball>())
        .iter(world)
        .map(|position| **position)
        .nth(0)
        .expect("No ball found")
        
}