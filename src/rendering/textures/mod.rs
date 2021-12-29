mod sampler;
mod regions;
mod resources;

pub use image::*;
use legion::systems::CommandBuffer;
pub use sampler::*;
pub use regions::*;
pub use resources::*;

use crate::prelude::*;

#[derive(Debug)]
pub enum TextureError {
    ImageError,
    TextureCreationError
}

#[derive(Debug, Clone)]
pub struct Texture(pub String);

impl Texture {
    pub fn png(name: &str) -> Self {
        let mut name = name.to_owned();
        name.push_str(".png");
        Self(name)
    }
}        

impl Deref for Texture {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn set_texture(buffer: &mut CommandBuffer, entity: Entity, texture: String) {
    buffer.add_component(entity, Texture(texture));
    buffer.remove_component::<RenderGraphSet>(entity)
}