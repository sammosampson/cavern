use crate::prelude::*;

#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
#[read_component(NextPosition)]
#[read_component(Position)]
#[read_component(Heading)]
pub fn check_collision(buffer: &mut CommandBuffer, world: &mut SubWorld) {
    let balls: Vec<(Entity, NextPosition, Heading)> = <(Entity, &NextPosition, &Heading)>::query()
        .iter_mut(world)
        .map(|(ball, next_position, heading)| (*ball, *next_position, *heading))
        .collect();

    let (ball, next_position, heading)  = balls.first().unwrap();
    
    let distance_to_centre = (next_position.0.x - HALF_SCREEN_WIDTH).abs();
    let threshold = HALF_SCREEN_WIDTH - BAT_POSITION_OFFSET - HALF_BAT_WIDTH - BALL_RADIUS;

    if distance_to_centre >= threshold {
        let bat_index = if heading.0.x < 0.0 { 0 } else { 1 };
    
        let bats: Vec<Position> = <(&Position, &Bat)>::query()
            .iter(world)
            .filter(|(_, bat)| bat.0 == bat_index)
            .map(|(bat_position, _)| *bat_position)
            .collect();
        
        let bat_position = bats.first().unwrap();

        if next_position.0.y + BALL_RADIUS <= bat_position.0.y + HALF_BAT_WIDTH
            && next_position.0.y - BALL_RADIUS >= bat_position.0.y - HALF_BAT_WIDTH
        {
            buffer.add_component(*ball, create_bat_collision());
        }

    } else if next_position.0.y > BOUNDS_MAX_Y || next_position.0.y < BOUNDS_MIN_Y {
        buffer.add_component(*ball, create_wall_collision());
    }
}

#[system(for_each)]
pub fn handle_collision(
    entity: &Entity,
    position: &Position,
    collider: &Collision,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    let collided_with = **collider;

    if collided_with == CollidedWith::Bat {
        heading.0.x = -heading.0.x;
    } 
    else if collided_with == CollidedWith::Wall {
        heading.0.y = -heading.0.y;
    }
    
    buffer.remove_component::<Collision>(*entity);
    buffer.push(create_impact(game_timer, *position));
}

#[system(for_each)]
#[filter(component::<Bat>())]
pub fn contain_bat_in_bounds(
    position: &Position,
    heading: &mut Heading,
) {
    if position.0.y + HALF_BAT_HEIGHT > BOUNDS_MAX_Y && heading.0.y > 0.0 {
        heading.0 = Vector::default();
    }

    if position.0.y - HALF_BAT_HEIGHT < BOUNDS_MIN_Y && heading.0.y < 0.0{
        heading.0 = Vector::default();
    }
}