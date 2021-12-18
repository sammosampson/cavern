use crate::prelude::*;

#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
#[read_component(NextPosition)]
#[read_component(Position)]
#[read_component(Heading)]
pub fn check_collision(buffer: &mut CommandBuffer, world: &mut SubWorld) {
    let balls: Vec<(Entity, Position, NextPosition, Heading)> = <(Entity, &Position, &NextPosition, &Heading)>::query()
        .filter(component::<Ball>())
        .iter_mut(world)
        .map(|(ball, position, next_position, heading)| (*ball, *position, *next_position, *heading))
        .collect();

    if balls.len() == 0 {
        return;
    }

    let (ball, position, next_position, heading)  = balls.first().unwrap();
    let next_position = **next_position;
    let position = **position;
    let last_ball_distance_to_centre = ball_distance_to_centre(position);
    let next_ball_distance_to_centre = ball_distance_to_centre(next_position);
    let bat_threshold = HALF_SCREEN_WIDTH - BAT_POSITION_OFFSET - HALF_BAT_WIDTH;
    let goal_threshold = HALF_SCREEN_WIDTH;
    let bat_index = if heading.0.x < 0.0 { 0 } else { 1 };
    
    if next_ball_distance_to_centre >= goal_threshold {
        buffer.add_component(*ball, create_goal_collision(bat_index));
        return
    }
        
    if last_ball_distance_to_centre < bat_threshold && next_ball_distance_to_centre > bat_threshold {    
        let bats: Vec<Position> = <(&Position, &Bat)>::query()
            .iter(world)
            .filter(|(_, bat)| ***bat == bat_index)
            .map(|(bat_position, _)| *bat_position)
            .collect();
        
        let bat_position = bats.first().unwrap();
        let ball_top_extent = next_position.y + BALL_RADIUS;
        let bat_top_extent = bat_position.y + HALF_BAT_HEIGHT;
        let ball_bottom_extent = next_position.y - BALL_RADIUS;
        let bat_bottom_extent = bat_position.y - HALF_BAT_HEIGHT;
        
        if ball_top_extent <= bat_top_extent && ball_bottom_extent >= bat_bottom_extent {
            buffer.add_component(*ball, create_bat_collision());
        }
        return;
    } 
    
    if next_position.y > BOUNDS_MAX_Y || next_position.y < BOUNDS_MIN_Y {
        buffer.add_component(*ball, create_wall_collision());
    }
}

fn ball_distance_to_centre(position: Vector) -> f32 {
    (position.x - HALF_SCREEN_WIDTH).abs() + BALL_RADIUS
}

#[system(for_each)]
pub fn handle_collision(
    entity: &Entity,
    position: &mut Position,
    collider: &Collision,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    let collided_with = **collider;

    if collided_with == CollidedWith::Bat {
        heading.0.x = -heading.0.x;
        buffer.push(create_impact(game_timer, *position));
    } 
    else if collided_with == CollidedWith::Wall {
        heading.0.y = -heading.0.y;
        buffer.push(create_impact(game_timer, *position));
    } else {
        position.0 = Vector::new(HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT);
    }
    buffer.remove_component::<Collision>(*entity);    
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