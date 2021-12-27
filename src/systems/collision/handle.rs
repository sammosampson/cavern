use crate::prelude::*;

#[system(for_each)]
#[filter(component::<WallCollision>())]
pub fn handle_wall_collision(
    entity: &Entity,
    position: &mut Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    invert_ball_vertical_heading(heading);
    add_impact_effect(buffer, game_timer, *position);
    add_wall_impact_sound(buffer);
    remove_wall_collision(buffer, entity);
}

#[system(for_each)]
pub fn handle_bat_collision(
    entity: &Entity,
    collision: &BatCollision,
    maximum_velocity: &mut MaximumVelocity,
    position: &mut Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    invert_ball_horizontal_heading(heading);
    increase_ball_speed(maximum_velocity);
    add_impact_effect(buffer, game_timer, *position);
    set_bat_hit_effect(buffer, game_timer, collision.bat, collision.index);
    remove_bat_collision(buffer, entity);
}

fn increase_ball_speed(maximum_velocity: &mut MaximumVelocity) {
    **maximum_velocity += 10.0;
}

fn invert_ball_vertical_heading(heading: &mut Heading) {
    **heading = heading.to_y_inverted();
}

fn invert_ball_horizontal_heading(heading: &mut Heading) {
    **heading = heading.to_x_inverted();
}

fn remove_wall_collision(buffer: &mut CommandBuffer, entity: &Entity) {
    buffer.remove_component::<WallCollision>(*entity); 
}

fn remove_bat_collision(buffer: &mut CommandBuffer, entity: &Entity) {
    buffer.remove_component::<BatCollision>(*entity); 
}