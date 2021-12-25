use crate::prelude::*;

pub fn add_wall_impact_sound(buffer: &mut CommandBuffer) {
    buffer.push(create_wall_impact_sound());   
}

fn create_wall_impact_sound() -> (WorldEntityId, Sound) {(
    "WallImpactSound".into(),
    SoundResources::WallImpact.into()
)}

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

    animation.add_frame(TextureResources::Impact0);
    animation.add_frame(TextureResources::Impact1);
    animation.add_frame(TextureResources::Impact2);
    animation.add_frame(TextureResources::Impact3);
    animation.add_frame(TextureResources::Impact4);

    animation
}
