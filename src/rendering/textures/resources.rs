use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureResources {
    Ball,
    Table,
    Bat00,
    Bat10,
    Impact0,
    Impact1,
    Impact2,
    Impact3,
    Impact4,
    Effect0,
    Effect1
}

pub fn initialise_texture_cache(textures: &mut TextureCache, screen_renderer: &ScreenRenderer) -> Result<(), TextureError> {
    textures.insert(screen_renderer, TextureResources::Ball, &include_bytes!("../../../images/ball.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Table, &include_bytes!("../../../images/table.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat00, &include_bytes!("../../../images/bat00.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat10, &include_bytes!("../../../images/bat10.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact0, &include_bytes!("../../../images/impact0.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact1, &include_bytes!("../../../images/impact1.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact2, &include_bytes!("../../../images/impact2.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact3, &include_bytes!("../../../images/impact3.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact4, &include_bytes!("../../../images/impact4.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Effect0, &include_bytes!("../../../images/effect0.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Effect1, &include_bytes!("../../../images/effect1.png")[..])?; 
    Ok(())
}

#[derive(Default)]
pub struct TextureCache {
    inner: HashMap<TextureResources, SamplerTexture>
}

impl TextureCache {
    fn insert(&mut self, screen_renderer: &ScreenRenderer, texture: TextureResources, data: &[u8]) -> Result<(), TextureError> {
        self.inner.insert(texture, SamplerTexture::new(&screen_renderer.display, data)?);
        Ok(())
    }

    pub fn get(&self, texture: &TextureResources) -> Option<&SamplerTexture> {
        self.inner.get(texture)
    }
}