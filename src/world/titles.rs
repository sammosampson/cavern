
use crate::prelude::*;

pub fn add_press_space_title(buffer: &mut CommandBuffer, game_timer: &GameTimer) {
    buffer.push((
        create_press_space_title_animation(game_timer), 
        Layer(3), 
        Position(Vector::new(HALF_SCREEN_WIDTH, 280.0)), 
        WorldEntityId::from("PressSpace"),
        MenuScreenItem
    ));
}

fn create_press_space_title_animation(game_timer: &GameTimer) -> Animation {
    let mut animation = create_animation(
        Duration::from_secs(1).as_secs_f32(), 
        game_timer.total_game_time());

    for frame_number in 0..=9 { 
        let mut frame_name = "space".to_string();
        frame_name.push_str(&frame_number.to_string());
        animation.add_frame(Texture::png(&frame_name));
    }

    animation
}