use crate::prelude::*;

pub fn add_wall_impact_sounds(buffer: &mut CommandBuffer) {
    buffer.push(create_sound_components(SoundResources::Bounce(random_number(0..4))));
    buffer.push(create_sound_components(SoundResources::BounceSynth));   
}

pub fn add_impact_effect(buffer: &mut CommandBuffer, game_timer: &GameTimer, position: Position) {
    buffer.push(create_impact_effect(game_timer, position));   
}

fn create_impact_effect(game_timer: &GameTimer, position: Position) -> (WorldEntityId, Animation, Effect, Layer, Position) {(
    create_impact_effect_id(game_timer),
    create_impact_effect_animation(game_timer),
    Effect,
    Layer(1),
    position
)}

fn create_impact_effect_id(game_timer: &GameTimer) -> WorldEntityId {
    format!("Impact{:?}", game_timer.total_game_time()).into()
}

fn create_impact_effect_animation(game_timer: &GameTimer) -> Animation {
    let mut animation = create_animation(
        Duration::from_millis(333).as_secs_f32(), 
        game_timer.total_game_time());

    animation.add_frame(TextureResources::Impact(0));
    animation.add_frame(TextureResources::Impact(1));
    animation.add_frame(TextureResources::Impact(2));
    animation.add_frame(TextureResources::Impact(3));
    animation.add_frame(TextureResources::Impact(4));

    animation
}
