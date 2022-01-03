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
    let mut to_render: Vec<_> = <(&WorldEntityId, &Texture, &Position, &Layer)>::query()
        .iter(world)
        .map(|item| RenderItem::from(item))
        .collect();

    to_render.sort_by(| item_a, item_b | 
        item_a
            .layer.cmp(&item_b.layer)
            .then(item_a.texture.cmp(&item_b.texture)));
    
    let mut renderers = vec!();
    let mut instances = vec!();
    let mut last_texture = "".to_string();

    for item in to_render {
        if last_texture != item.texture {
            last_texture = item.texture.clone(); 
        }

        renderers.last_mut().unwrap().add_instance(&item);
    }
    
    screen_renderer
        .render(&renderers, textures)
        .expect("Could not render");
}

/*
   let renderer = InstanceRenderer::new(screen_renderer, textures, &item.texture)
                .expect("Could not create renderer");
            renderers.push(renderer);
          */