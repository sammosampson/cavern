mod shaders;
mod screen;
mod display;

pub use glium:: {
    uniform,
    implement_vertex,
    ProgramCreationError,
    Surface,
    Depth,
    DrawParameters,
    draw_parameters,
    Blend,
    Program,
    Frame,
    Vertex,
    VertexBuffer,
    index::NoIndices,
    texture::SrgbTexture2d,
    backend::glutin::Display,
    backend::glutin::DisplayCreationError,
    texture::RawImage2d,
    uniforms::*,
    glutin:: {
        event::*,
        ContextBuilder,
        event_loop::*,
        window::*,
        platform::run_return::EventLoopExtRunReturn,
        event_loop::EventLoop,
        event::Event
    }
};

pub use display::*;
pub use shaders::*;
pub use screen::*;

use crate::prelude::*;

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders,
    BufferSwapError,
    BufferCreationError,
    TextureLoadError,
    DrawError
}

pub const SCREEN_WIDTH: f32 = 800.0;
pub const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH * 0.5;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT * 0.5;

pub fn centre_screen() -> Vector {
    Vector::new(HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT)
}

#[derive(Copy, Clone, Debug)]
pub struct Layer(pub u8);

impl Deref for Layer {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
