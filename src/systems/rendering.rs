use crate::prelude::*;

#[system(simple)]
#[read_component(WorldEntityId)]
#[read_component(Texture)]
#[read_component(Position)]
#[read_component(Layer)]
#[read_component(Instanced)]
pub fn render(
    world: &mut SubWorld,
    #[resource] screen_renderer: &mut ScreenRenderer,
    #[resource] textures: &TextureCache
) {
    let mut to_render: Vec<ItemRendererItem> = <(&WorldEntityId, &Texture, &Position, &Layer)>::query()
        .iter(world)
        .map(|(id, texture, position, layer)| ItemRendererItem::new(
            screen_renderer,
            textures, 
            id.clone(), 
            &**texture, 
            **position, 
            **layer)
            .expect("Could not setup render"))
        .collect();

    to_render.sort_by(| item_a, item_b | item_a.layer().cmp(&item_b.layer()));
        
    screen_renderer
        .render(&to_render, textures)
        .expect("Could not render");
}
