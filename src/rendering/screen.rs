use crate::prelude::*;

pub struct ScreenRenderer {
    pub display: Display,
    editor_renderer: EditorRenderer
    
}

impl ScreenRenderer {
    pub fn process_event(&mut self, event: &WindowEvent) {
        self.editor_renderer.process_event(event);
    }

    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display_for_renderer(event_loop)?;
        let editor_renderer = EditorRenderer::new(&display);
        Ok(Self {
            display,
            editor_renderer
        })
    }

    pub fn start_render(&mut self) -> Frame {
        let mut target = self.create_draw_target();        
        clear_target_color_and_depth(&mut target);
        target
    }

    fn create_draw_target(&self) -> Frame {
        self.display.draw()
    }
}

fn create_display_for_renderer(event_loop: &EventLoop<()>) -> Result<Display, RendererError> {
    Ok(create_display(event_loop).map_err(|_|RendererError::FailedToDisplayWindow)?)
}

fn clear_target_color_and_depth(target: &mut Frame) {
    target.clear_color_and_depth((0.3, 0.3, 0.5, 1.0), 1.0);
}

pub fn complete_screen_render(target: Frame) -> Result<(), RendererError> {
    Ok(
        target.finish().map_err(|_|RendererError::BufferSwapError)?
    )
}
