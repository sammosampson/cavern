mod sampler;
mod regions;
mod resources;

pub use image::*;
pub use sampler::*;
pub use regions::*;
pub use resources::*;

#[derive(Debug)]
pub enum TextureError {
    ImageError,
    TextureCreationError
}

#[derive(Debug, Clone, Copy)]
pub struct Texture(pub TextureResources);