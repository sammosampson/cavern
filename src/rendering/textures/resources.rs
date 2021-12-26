use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureResources {
    Ball,
    Table,
    Bat0(u8),
    Bat1(u8),
    Impact(u8),
    Effect(u8),
    Menu(u8),
}

pub fn initialise_texture_cache(textures: &mut TextureCache, screen_renderer: &ScreenRenderer) -> Result<(), TextureError> {
    textures.insert(screen_renderer, TextureResources::Ball, &include_bytes!("../../../images/ball.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Table, &include_bytes!("../../../images/table.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat0(0), &include_bytes!("../../../images/bat00.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat0(1), &include_bytes!("../../../images/bat01.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat0(2), &include_bytes!("../../../images/bat02.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat1(0), &include_bytes!("../../../images/bat10.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat1(1), &include_bytes!("../../../images/bat11.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Bat1(2), &include_bytes!("../../../images/bat12.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact(0), &include_bytes!("../../../images/impact0.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact(1), &include_bytes!("../../../images/impact1.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact(2), &include_bytes!("../../../images/impact2.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact(3), &include_bytes!("../../../images/impact3.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Impact(4), &include_bytes!("../../../images/impact4.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Effect(0), &include_bytes!("../../../images/effect0.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Effect(0), &include_bytes!("../../../images/effect0.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Menu(0), &include_bytes!("../../../images/menu0.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Menu(1), &include_bytes!("../../../images/menu1.png")[..])?; 
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