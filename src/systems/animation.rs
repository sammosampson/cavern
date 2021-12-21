use crate::prelude::*;

#[system(for_each)]
pub fn render_animation_frame(
    entity_id: &WorldEntityId,
    animation: &Animation,
    position: &Position,
    #[resource] game_timer: &GameTimer,
    #[resource] screen_renderer: &ScreenRenderer,
    #[resource] item_renderer: &mut ItemRenderer,
) {
    if let Some(frame) = animation.get_frame(game_timer.total_game_time()) {
        let frame_texture = animation.get_frame_texture(frame);
        if frame == 0 {
            item_renderer.add_item_to_render(
                &screen_renderer,
                entity_id,
                frame_texture,
                position.0, 
                1).unwrap();
        } else {
            let item = item_renderer.find_mut(entity_id).unwrap();
            item.set_centre_position(position.0);
            item.set_texture(&screen_renderer, frame_texture).unwrap();
        }
    } else {
        item_renderer.remove_item_to_render(entity_id);
    }
}