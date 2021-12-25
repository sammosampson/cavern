use crate::prelude::*;

pub fn create_item_renderer() -> ItemRenderer {
    ItemRenderer::default()
}


#[derive(Default)]
pub struct ItemRenderer {
    render_items: HashMap<WorldEntityId, ItemRendererItem>
}

impl ItemRenderer {
    pub fn add_item_to_render(
        &mut self,
        screen_renderer: &ScreenRenderer,
        textures: &TextureCache,
        entity_id: &WorldEntityId,
        texture: TextureResources,
        centre_position: Vector,
        layer: u8
    ) -> Result<(), RendererError> {
        self.render_items.insert(
            entity_id.clone(), 
            ItemRendererItem::new(screen_renderer, textures, entity_id.clone(), texture, centre_position, layer)?
        );
        Ok(())
    }

    pub fn remove_item_to_render(&mut self, entity_id: &WorldEntityId) {
        self.render_items.remove(entity_id);
    }

    pub fn render(&self, target: &mut Frame, textures: &TextureCache) -> Result<(), RendererError> {
        let mut items: Vec<_> = self.render_items.iter().collect();
        items.sort_by(|a , b| a.1.layer().cmp(&b.1.layer()));
        for (_, render_item) in items {
            render_item.render(target, textures)?;
        }
        Ok(())
    }

    pub fn find_mut(&mut self, entity_id: &WorldEntityId) -> Option<&mut ItemRendererItem> {
        self.render_items.get_mut(entity_id)
    }

    pub fn contains(&self, entity_id: &WorldEntityId) -> bool {
        self.render_items.contains_key(entity_id)
    }
}