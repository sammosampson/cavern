use crate::prelude::*;

pub struct ScreenRenderer {
    pub display: Display,
}

impl ScreenRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        Ok(Self {
            display: create_display_for_renderer(event_loop)?,
        })
    }

    pub fn render(&mut self, render_items: &Vec<ItemRendererItem>, textures: &TextureCache) -> Result<(), RendererError> {
        let mut target = self.create_draw_target();        
        clear_target_color_and_depth(&mut target);
        self.render_items(render_items, textures, &mut target)?;
        complete_target_draw(target)?;
        Ok(())
    }

    fn create_draw_target(&self) -> Frame {
        self.display.draw()
    }

    fn render_items(&self, render_items: &Vec<ItemRendererItem>, textures: &TextureCache, target: &mut Frame) -> Result<(), RendererError> {
        for item in render_items {
            item.render(target, textures)?;
        }
        Ok(())
    }
}

fn create_display_for_renderer(event_loop: &EventLoop<()>) -> Result<Display, RendererError> {
    Ok(create_display(event_loop).map_err(|_|RendererError::FailedToDisplayWindow)?)
}

fn clear_target_color_and_depth(target: &mut Frame) {
    target.clear_color_and_depth((0.3, 0.3, 0.5, 1.0), 1.0);
}

fn complete_target_draw(target: Frame) -> Result<(), RendererError> {
    Ok(
        target.finish().map_err(|_|RendererError::BufferSwapError)?
    )
}
