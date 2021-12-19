use crate::prelude::*;

const GOAL_THRESHOLD: f32 = HALF_SCREEN_WIDTH;
const BAT_THRESHOLD: f32 = HALF_SCREEN_WIDTH - BAT_POSITION_OFFSET - HALF_BAT_WIDTH;
    
#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
#[read_component(NextPosition)]
#[read_component(Position)]
#[read_component(Heading)]
pub fn check_collision(buffer: &mut CommandBuffer, world: &mut SubWorld
) {
    let balls = get_balls(world);
    
    if balls.len() == 0 {
        return;
    }

    let (ball, position, next_position, heading) = balls.first().unwrap();
    let bat_index = bat_index(**heading);
    let ball_last_distance_to_centre = ball_distance_to_centre(**position);
    let ball_next_distance_to_centre = ball_distance_to_centre(**next_position);
    
    if ball_next_distance_to_centre >= GOAL_THRESHOLD {
        buffer.add_component(*ball, create_goal_collision(bat_index));
        return;
    }
        
    if ball_last_distance_to_centre < BAT_THRESHOLD && ball_next_distance_to_centre > BAT_THRESHOLD {    
        let bat_position = get_bat_position(world, bat_index);
        let (ball_top_extent, ball_bottom_extent) = vertical_ball_extents(**next_position);
        let (bat_top_extent, bat_bottom_extent) = vertical_bat_extents(bat_position);
        
        if ball_top_extent <= bat_top_extent && ball_bottom_extent >= bat_bottom_extent {
            buffer.add_component(*ball, create_bat_collision());
        }
        return;
    } 
    
    if next_position.y > BOUNDS_MAX_Y || next_position.y < BOUNDS_MIN_Y {
        buffer.add_component(*ball, create_wall_collision());
    }
}

fn get_balls(world: &SubWorld) -> Vec<(Entity, Position, NextPosition, Heading)> {
    let balls: Vec<(Entity, Position, NextPosition, Heading)>  = <(Entity, &Position, &NextPosition, &Heading)>::query()
        .filter(component::<Ball>())
        .iter(world)
        .map(|(ball, position, next_position, heading)| (*ball, *position, *next_position, *heading))
        .collect();
    balls
}

fn ball_distance_to_centre(position: Vector) -> f32 {
    (position.x - HALF_SCREEN_WIDTH).abs() + BALL_RADIUS
}

fn bat_index(heading: Vector) -> u8 {
    if heading.x < 0.0 { 0 } else { 1 }
}

fn get_bat_position(world: &SubWorld, bat_index: u8) -> Vector {
    let bats: Vec<Position> = <(&Position, &Bat)>::query()
        .iter(world)
        .filter(|(_, bat)| ***bat == bat_index)
        .map(|(bat_position, _)| *bat_position)
        .collect();

    **bats.first().unwrap()
}

fn vertical_ball_extents(position: Vector) -> (f32, f32) {
    (position.y + BALL_RADIUS, position.y - BALL_RADIUS)
}

fn vertical_bat_extents(position: Vector) -> (f32, f32) {
    (position.y + HALF_BAT_HEIGHT, position.y - HALF_BAT_HEIGHT) 
}
