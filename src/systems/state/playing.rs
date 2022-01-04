use crate::prelude::*;

#[system(simple)]
#[read_component(MenuScreenItem)]
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
            let level = load_level(0.into());
            draw_level(buffer, &level);
            remove_menu_screen_items(buffer, world);
        },
        GameStatus::Finishing => {
            remove_game_over_screen(buffer, world);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

fn remove_menu_screen_items(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<MenuScreenItem>())
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
