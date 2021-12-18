use crate::prelude::*;

pub fn create_display(event_loop: &EventLoop<()>) -> Result<Display, DisplayCreationError> {
    Display::new(WindowBuilder::new().with_maximized(true), ContextBuilder::new().with_depth_buffer(24), event_loop)
}