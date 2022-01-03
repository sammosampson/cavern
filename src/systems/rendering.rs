use crate::prelude::*;

#[system(simple)]
#[read_component(WorldEntityId)]
#[read_component(Texture)]
#[read_component(Position)]
#[read_component(Layer)]
pub fn render(
    world: &mut SubWorld,
    #[resource] screen_renderer: &mut ScreenRenderer,
    #[resource] textures: &TextureCache
) {
    let renderers = create_renderers(
        screen_renderer,
        textures,
        get_render_items(world));
    
    render_items(screen_renderer, textures, &renderers);
}

fn get_render_items(world: &mut SubWorld) -> Vec<RenderItem> {
    let to_render: Vec<RenderItem> = <(&WorldEntityId, &Texture, &Position, &Layer)>::query()
        .iter(world)
        .map(|item| RenderItem::from(item))
        .collect();

    to_render
}

fn create_renderers(screen_renderer: &ScreenRenderer, textures: &TextureCache, to_render: Vec<RenderItem>) -> InstanceRenderers {    
    InstanceRenderers::new(screen_renderer, textures, to_render)
        .expect("cannot create instance renderers")
}

fn render_items(screen_renderer: &mut ScreenRenderer, textures: &TextureCache, renderers: &InstanceRenderers) {
    screen_renderer
        .render(&renderers, textures)
        .expect("Could not render");
}