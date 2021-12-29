use crate::prelude::*;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn initialise_texture_cache(textures: &mut TextureCache, screen_renderer: &ScreenRenderer) -> Result<(), TextureError> {   

    let paths = fs::read_dir("./images").unwrap();

    for path in paths {
        let path = path.unwrap();
        let file = File::open(path.path()).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut data: Vec<u8> = vec!();
        buf_reader.read_to_end(&mut data).unwrap();

        textures.insert(screen_renderer, path.file_name().to_str().unwrap().to_string(), &data).unwrap();
    }

    Ok(())
}

#[derive(Default)]
pub struct TextureCache {
    inner: HashMap<String, SamplerTexture>
}

impl TextureCache {
    fn insert(&mut self, screen_renderer: &ScreenRenderer, texture: String, data: &[u8]) -> Result<(), TextureError> {
        println!("caching {:?}", texture);
        self.inner.insert(texture, SamplerTexture::new(&screen_renderer.display, data)?);
        Ok(())
    }

    pub fn get(&self, texture: &str) -> Option<&SamplerTexture> {
        self.inner.get(texture)
    }
}