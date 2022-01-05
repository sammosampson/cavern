mod rendering;
mod state;
mod events;
mod world;
mod time;
mod animation;
mod input;
mod audio;
mod physics;

pub use legion::*;
pub use legion::query::Query;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;

pub fn build_world() -> World {
    let world = World::default();
    world
}

pub fn build_start_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_starting_system())
        .add_system(events::proliferate_system_events_system())
        .flush()    
        .add_system(time::game_time_system())    
        .add_system(input::title_screen_input_system())
        .flush()
        .add_thread_local(animation::render_first_animation_frame_system())
        .add_thread_local(animation::render_animation_frame_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())    
        .build()
}

pub fn build_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_playing_system())
        .flush()
        .add_system(events::proliferate_system_events_system())
        .add_system(physics::initialise_movement_system())
        .flush()
        .add_system(time::game_time_system())    
        .add_system(world::set_player_direction_from_input_system())
        .flush()
        .add_system(physics::set_heading_from_direction_system())
        .add_system(physics::reset_heading_with_no_direction_system())
        .add_system(physics::set_velocity_given_heading_system())
        .add_system(physics::apply_velocity_to_position_system())
        .flush()
        .add_thread_local(physics::set_position_system())
        .flush()
        .add_thread_local(animation::render_first_animation_frame_system())
        .add_thread_local(animation::render_animation_frame_system())
        .flush()
        .add_thread_local(rendering::render_system())
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())    
        .build()
}

pub fn build_finish_schedule() -> Schedule {
    Schedule::builder()
        .add_system(state::transition_state_to_finishing_system())
        .add_system(events::proliferate_system_events_system())
        .flush()
        .add_system(input::title_screen_input_system())
        .flush()    
        .add_thread_local(rendering::render_system())
        .add_system(audio::play_music_system())
        .add_system(audio::play_sound_system())
        .flush()
        .add_thread_local(world::remove_entity_system())
        .add_system(state::exit_if_requested_system())
        .add_system(events::destroy_system_events_system())    
        .build()
}