use crate::prelude::*;

#[system(for_each)]
pub fn render_animation_frame(
    entity: &Entity,
    entity_id: &WorldEntityId,
    animation: &Animation,
    layer: &Layer,
    position: &Position,
    buffer: &mut CommandBuffer,
    #[resource] game_timer: &GameTimer,
    #[resource] screen_renderer: &ScreenRenderer,
    #[resource] textures: &TextureCache,
    #[resource] item_renderer: &mut ItemRenderer,
) {
    println!("animating {:?}", entity_id);
    if let Some(frame) = animation.get_frame(game_timer.total_game_time()) {
        let frame_texture = animation.get_frame_texture(frame);
        if frame == 0 {
            item_renderer
                .add_item_to_render(
                    screen_renderer,
                    textures,
                    entity_id,
                    frame_texture,
                    position.0, 
                    layer.0)
                .expect("Could not add item to render");
        } else {
            let item = item_renderer
                .find_mut(entity_id)
                .expect("Could find item to render");
            
            item.set_centre_position(position.0);
            item.set_texture(frame_texture);
        }
    } else {
        buffer.add_component(*entity, Remove);
    }
}