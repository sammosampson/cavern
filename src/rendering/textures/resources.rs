use crate::prelude::*;

#[derive(Default)]
pub struct TextureCache {
    inner: HashMap<String, SamplerTexture>
}

impl TextureCache {
    fn insert(
        &mut self, screen_renderer: &ScreenRenderer,
        texture: String,
        data: &[u8]
    ) -> Result<(), TextureError> {
        println!("making sampler named: {:?}", texture);
        self.inner.insert(texture, SamplerTexture::new(&screen_renderer.display, data)?);
        Ok(())
    }

    pub fn get(&self, texture: &str) -> Option<&SamplerTexture> {
        self.inner.get(texture)
    }
}

pub fn initialise_texture_cache(
    textures: &mut TextureCache,
    screen_renderer: &ScreenRenderer
) -> Result<(), TextureError> {   

    for texture_file in read_texture_files()? {
        let (texture_file_name, texture_file_content) = texture_file?;
        textures.insert(screen_renderer, texture_file_name, &texture_file_content)?;
    }
    Ok(())
}

fn read_texture_files() -> Result<FolderFileIterator, TextureError> {
    Ok(read_files_in_folder("./images")?)
}