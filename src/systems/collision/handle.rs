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
    **heading = heading.to_y_inverted();
    add_impact_effect(buffer, game_timer, *position);
    add_wall_impact_sound(buffer);
    buffer.remove_component::<WallCollision>(*entity);    
}

#[system(for_each)]
pub fn handle_bat_collision(
    entity: &Entity,
    collision: &BatCollision,
    position: &mut Position,
    heading: &mut Heading,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
) {
    **heading = heading.to_x_inverted();
    add_impact_effect(buffer, game_timer, *position);
    set_bat_hit_effect(buffer, game_timer, collision.bat, collision.index);
    buffer.remove_component::<BatCollision>(*entity);    
}