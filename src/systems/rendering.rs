use crate::prelude::*;

#[system(simple)]
#[read_component(WorldEntityId)]
#[read_component(Texture)]
#[read_component(Position)]
#[read_component(Layer)]
pub fn render(
    world: &mut SubWorld,
    #[resource] event_producer: &mut SystemEventProducer,
    #[resource] editor_renderer: &mut EditorRenderer,
    #[resource] editor_graph: &mut EditorGraph,
    #[resource] screen_renderer: &mut ScreenRenderer,
    #[resource] textures: &TextureCache
) {
    let renderers = create_renderers(
        screen_renderer,
        textures,
        get_render_items(world));
    
    render_items(
        screen_renderer, 
        event_producer,
        editor_graph,
        editor_renderer,
        textures, 
        &renderers);
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

fn render_items(
    screen_renderer: &mut ScreenRenderer, 
    event_producer: &mut SystemEventProducer,
    editor_graph: &mut EditorGraph,
    editor_renderer: &mut EditorRenderer,
    textures: &TextureCache, 
    renderers: &InstanceRenderers
) {
    let mut target = screen_renderer.start_render();
    
    renderers.render(textures, &mut target)
        .expect("Could not render textures");
    
    if editor_renderer.render(editor_graph, event_producer, &screen_renderer.display, &mut target) {
        screen_renderer.display.gl_window().window().request_redraw();
    }
    
    complete_screen_render(target)
        .expect("Could not complete render");
}