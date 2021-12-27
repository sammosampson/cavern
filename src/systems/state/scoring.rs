use crate::prelude::*;

#[system(simple)]
#[read_component(Ball)]
#[read_component(Bat)]
#[read_component(ScoreBoard)]
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
                remove_ball(buffer, world);
                add_arena_score_effect(buffer, game_timer, index);
                set_losing_bat_score_texture(buffer, world, index);
                increment_score(score, index);
                display_score(index, score, game_timer, buffer, world);
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

fn increment_score(score: &mut PlayerScore, index: PlayerIndex) {
    score.increment(index)
}

fn display_score(
    index: PlayerIndex,
    player_score: &PlayerScore,
    game_timer: &GameTimer,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {
    <(Entity, &ScoreBoard)>::query()
        .iter(world)
        .for_each(|(entity, board)| {
            if index == **board {
                set_win_score_texture(buffer, *entity, player_score.get(index), index);
            } else {
                let index = index.opposing();
                set_lose_score_animation(buffer, *entity, game_timer, player_score.get(index), index);
            }
        });
}

fn remove_ball(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<Ball>())
        .iter(world)
        .for_each(|entity| remove_entity(buffer, *entity));
}

fn set_losing_bat_score_texture(buffer: &mut CommandBuffer, world: &SubWorld, index: PlayerIndex) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .filter(|(_, bat)| ***bat == index)
        .for_each(|(entity, _)|{
            set_bat_score_texture(buffer, *entity, index);
        });
}
