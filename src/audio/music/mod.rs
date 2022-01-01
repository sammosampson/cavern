mod resources;

pub use resources::*;
use crate::prelude::*;

pub fn create_music_components(resource: &str) -> (Music, ) {
    (Music(ogg(resource)), )
}

#[derive(Debug, Clone)]
pub struct Music(pub String);

impl Deref for Music {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} 
