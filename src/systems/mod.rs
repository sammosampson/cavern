mod rendering;
mod events;
mod game_state;
mod movement;
mod time;
mod collision;
mod animation;
mod sound;

pub use legion::*;
pub use legion::query::Query;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;
use crate::prelude::*;

pub fn build_world() -> World {
    let mut world = World::default();
    add_items_to_world(&mut world);
    world
}

pub fn build_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(events::proliferate_system_events_system())
        .add_system(movement::initialise_movement_system())
        .flush()
        .add_system(time::calculate_elapsed_time_system())
        .add_thread_local(rendering::build_play_render_graph_system())
        .add_system(movement::set_heading_from_input_system())
        .add_system(collision::contain_bat_in_bounds_system())
        .add_system(movement::set_velocity_given_heading_system())
        .add_system(movement::apply_velocity_to_position_system())
        .add_system(collision::check_collision_system())
        .flush()
        .add_system(collision::handle_wall_collision_system())
        .add_system(collision::handle_bat_collision_system())
        .add_system(collision::handle_ball_in_goal_system())
        .flush()
        .add_thread_local(movement::movement_system())
        .flush()
        .add_thread_local(animation::render_animation_frame_system())
        .add_thread_local(rendering::render_system())
        .add_system(sound::play_sound_system())
        .add_system(game_state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())
        .build()
}