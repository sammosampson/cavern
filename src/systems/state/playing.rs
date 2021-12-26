use crate::prelude::*;

#[system(simple)]
#[read_component(Bat)]
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
        GameStatus::Scoring(_) => {
            set_normal_bat_textures(buffer, world);
            add_ball(buffer);
        },
        GameStatus::Starting => {
            remove_menu_screen(buffer, world);
            add_ball(buffer);
        },
        GameStatus::Finishing => {
            remove_game_over_screen(buffer, world);
            add_ball(buffer);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

pub fn set_normal_bat_textures(buffer: &mut CommandBuffer, world: &SubWorld) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .for_each(|(entity, bat)|{
            set_normal_bat_texture(buffer, *entity, bat.0);
        });
}

pub fn remove_menu_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<MenuScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}

pub fn remove_game_over_screen(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<GameOverScreen>())
        .iter(world)
        .for_each(|entity| {
            remove_entity(buffer, *entity);
        });
}
