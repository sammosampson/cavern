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
    Digit0(u8),
    Digit1(u8),
    Digit2(u8),
    Over,
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
    textures.insert(screen_renderer, TextureResources::Effect(1), &include_bytes!("../../../images/effect1.png")[..])?;
    textures.insert(screen_renderer, TextureResources::Menu(0), &include_bytes!("../../../images/menu0.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Menu(1), &include_bytes!("../../../images/menu1.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(0), &include_bytes!("../../../images/digit00.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(1), &include_bytes!("../../../images/digit01.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(2), &include_bytes!("../../../images/digit02.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(3), &include_bytes!("../../../images/digit03.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(4), &include_bytes!("../../../images/digit04.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(5), &include_bytes!("../../../images/digit05.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(6), &include_bytes!("../../../images/digit06.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(7), &include_bytes!("../../../images/digit07.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(8), &include_bytes!("../../../images/digit08.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit0(9), &include_bytes!("../../../images/digit09.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(0), &include_bytes!("../../../images/digit10.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(1), &include_bytes!("../../../images/digit11.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(2), &include_bytes!("../../../images/digit12.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(3), &include_bytes!("../../../images/digit13.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(4), &include_bytes!("../../../images/digit14.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(5), &include_bytes!("../../../images/digit15.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(6), &include_bytes!("../../../images/digit16.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(7), &include_bytes!("../../../images/digit17.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(8), &include_bytes!("../../../images/digit18.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit1(9), &include_bytes!("../../../images/digit19.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(0), &include_bytes!("../../../images/digit20.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(1), &include_bytes!("../../../images/digit21.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(2), &include_bytes!("../../../images/digit22.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(3), &include_bytes!("../../../images/digit23.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(4), &include_bytes!("../../../images/digit24.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(5), &include_bytes!("../../../images/digit25.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(6), &include_bytes!("../../../images/digit26.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(7), &include_bytes!("../../../images/digit27.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(8), &include_bytes!("../../../images/digit28.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Digit2(9), &include_bytes!("../../../images/digit29.png")[..])?; 
    textures.insert(screen_renderer, TextureResources::Over, &include_bytes!("../../../images/over.png")[..])?; 
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