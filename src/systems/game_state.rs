use crate::prelude::*;

#[system(for_each)]
pub fn exit_if_requested(
    event: &SystemEvent, 
    #[resource] game_state: &mut GameState,
) {
    match event {
        SystemEvent::CloseRequested => game_state.transition_to(GameStatus::Exiting),
        _ => {}
    }
}

#[system(simple)]
pub fn transition_state_to_playing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
) {
    if game_state.has_entered() {
        return;
    }

    match game_state.previous_status() {
        GameStatus::Scoring(_) => {
            add_ball(buffer);
        },
        GameStatus::None => {
            add_arena(buffer);
            add_ball(buffer);
            add_bat0(buffer);
            add_bat1(buffer);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

#[system(simple)]
#[read_component(Ball)]
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
