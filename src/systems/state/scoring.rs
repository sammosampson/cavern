use crate::prelude::*;

#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
pub fn transition_state_to_scored(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {  
    if let GameStatus::Scoring(index) = game_state.status() {
        if game_state.has_entered() {
            if game_state.time_in_state(game_timer) >= 3.0 {
                game_state.transition_to(GameStatus::Playing)
            }
            return;
        }
    
        match game_state.previous_status() {
            GameStatus::Playing => {
                remove_ball(buffer, world);
                add_arena_score_effect(buffer, game_timer, index);
                set_losing_bat_score_texture(buffer, world, index);
            },
            _ => {},
        }
    
        game_state.enter(game_timer.total_game_time());
    }
}

pub fn remove_ball(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<Ball>())
        .iter(world)
        .for_each(|entity| buffer.add_component(*entity, Remove));
}

pub fn set_losing_bat_score_texture(buffer: &mut CommandBuffer, world: &SubWorld, goal_index: u8) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .filter(|(_, bat)| bat.0 == goal_index)
        .for_each(|(entity, _)|{
            set_bat_score_texture(buffer, *entity, goal_index);
        });
}
