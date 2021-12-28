mod resources;
pub use resources::*;

pub fn create_music_components(resource: MusicResources) -> (Music, ) {
    (Music(resource), )
}

#[derive(Debug, Copy, Clone)]
pub struct Music(pub MusicResources);
