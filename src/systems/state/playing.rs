use crate::prelude::*;

#[system(simple)]
#[read_component(MenuScreen)]
#[read_component(GameOverScreen)]
pub fn transition_state_to_playing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {
    if game_state.has_entered() {
        return;
    }

    match game_state.previous_status() {
        GameStatus::Starting => {
            remove_menu_screen(buffer, world);
        },
        GameStatus::Finishing => {
            remove_game_over_screen(buffer, world);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

fn remove_menu_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<MenuScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}

fn remove_game_over_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<GameOverScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}
