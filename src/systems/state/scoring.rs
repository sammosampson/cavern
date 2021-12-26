use crate::prelude::*;

#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
pub fn transition_state_to_scored(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    #[resource] score: &mut PlayerScore,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {  
    if let GameStatus::Scoring(index) = game_state.status() {
        if game_state.has_entered() {
            if game_state.time_in_state(game_timer) >= 3.0 {
                game_state.transition_to(get_next_state(score))
            }
            return;
        }
    
        match game_state.previous_status() {
            GameStatus::Playing => {
                increment_score(score, index);
                remove_ball(buffer, world);
                add_arena_score_effect(buffer, game_timer, index);
                set_losing_bat_score_texture(buffer, world, index);
            },
            _ => {},
        }
    
        game_state.enter(game_timer.total_game_time());
    }
}

fn get_next_state(score: &PlayerScore) -> GameStatus {
    if score.game_won() { 
        return GameStatus::Finishing
    }
    GameStatus::Playing
}

pub fn increment_score(score: &mut PlayerScore, index: u8) {
    score.increment(index)
}

fn remove_ball(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<Ball>())
        .iter(world)
        .for_each(|entity| buffer.add_component(*entity, Remove));
}

fn set_losing_bat_score_texture(buffer: &mut CommandBuffer, world: &SubWorld, goal_index: u8) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .filter(|(_, bat)| bat.0 == goal_index)
        .for_each(|(entity, _)|{
            set_bat_score_texture(buffer, *entity, goal_index);
        });
}
