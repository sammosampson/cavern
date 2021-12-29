use crate::prelude::*;

pub fn initialise_texture_cache(textures: &mut TextureCache, screen_renderer: &ScreenRenderer) -> Result<(), TextureError> {   
    Ok(())
}

#[derive(Default)]
pub struct TextureCache {
    inner: HashMap<String, SamplerTexture>
}

impl TextureCache {
    fn insert(&mut self, screen_renderer: &ScreenRenderer, texture: String, data: &[u8]) -> Result<(), TextureError> {
        self.inner.insert(texture, SamplerTexture::new(&screen_renderer.display, data)?);
        Ok(())
    }

    pub fn get(&self, texture: &str) -> Option<&SamplerTexture> {
        self.inner.get(texture)
    }
}