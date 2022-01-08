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
    let texture_render_items = get_texture_render_items(world);
    let texture_instance_renderers = create_texture_intance_renderers(screen_renderer, textures,texture_render_items);
    let mut target = start_rendering(screen_renderer);
    render_texture_instances(&texture_instance_renderers, textures, &mut target);
    render_editor(editor_renderer, editor_graph, event_producer, screen_renderer, &mut target);   
    complete_render(target);
}

fn get_texture_render_items(world: &mut SubWorld) -> Vec<TextureRenderItem> {
    let to_render: Vec<TextureRenderItem> = <(&WorldEntityId, &Texture, &Position, &Layer)>::query()
        .iter(world)
        .map(|item| TextureRenderItem::from(item))
        .collect();

    to_render
}

fn create_texture_intance_renderers(screen_renderer: &ScreenRenderer, textures: &TextureCache, to_render: Vec<TextureRenderItem>) -> TextureInstanceRenderers {    
    TextureInstanceRenderers::new(screen_renderer, textures, to_render)
        .expect("cannot create instance renderers")
}

fn start_rendering(screen_renderer: &mut ScreenRenderer) -> Frame {
    screen_renderer.start_render()
}

fn render_texture_instances(renderers: &TextureInstanceRenderers, textures: &TextureCache, target: &mut Frame) {
    renderers.render(textures, target)
    .expect("Could not render textures")
}

fn render_editor(
    editor_renderer: &mut EditorRenderer,
    editor_graph: &mut EditorGraph,
    event_producer: &mut SystemEventProducer,
    screen_renderer: &mut ScreenRenderer,
    target: &mut Frame
) {
    if editor_renderer.render(editor_graph, event_producer, &screen_renderer.display, target) {
        screen_renderer.display.gl_window().window().request_redraw();
    }
}

fn complete_render(target: Frame) {
    complete_screen_render(target)
        .expect("Could not complete render");
}