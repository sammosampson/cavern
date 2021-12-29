use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<RenderGraphSet>())]
pub fn build_render_graph(
    entity: &Entity, 
    entity_id: &WorldEntityId, 
    texture: &Texture,
    position: &Position,
    layer: &Layer,
    #[resource] screen_renderer: &mut ScreenRenderer,
    #[resource] textures: &TextureCache,
    #[resource] item_renderer: &mut ItemRenderer,
    buffer: &mut CommandBuffer,
) {
    let texture = &**texture;
    match item_renderer.find_mut(entity_id) {
        Some(item) => {
            item.set_texture(texture);
        }
        None => {
            item_renderer
                .add_item_to_render(
                    screen_renderer, 
                    textures,
                    entity_id, 
                    texture, 
                    **position, 
                    **layer
                )
                .expect(&format!("Could not add item to render {:?}", entity_id));
    
        }

    }
    
    buffer.add_component(*entity, RenderGraphSet);
}   

#[system(simple)]
pub fn render(
    #[resource] screen_renderer: &mut ScreenRenderer,
    #[resource] textures: &TextureCache,
    #[resource] item_renderer: &mut ItemRenderer
) {
    screen_renderer
        .render(item_renderer, textures)
        .expect("Could not render");
}

