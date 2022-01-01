use crate::prelude::*;

#[derive(Default)]
pub struct MusicSourceCache {
    inner: HashMap<String, RawSoundSource>
}

impl MusicSourceCache {
    fn insert(&mut self, name: String, data: &[u8]) {
        self.inner.insert(name, RawSoundSource { bytes: data.to_vec() });
    }

    pub fn get(&self, resource: &str) -> Option<&RawSoundSource> {
        self.inner.get(resource)
    }
}

pub fn initialise_music_cache(music_cache: &mut MusicSourceCache) -> Result<(), AudioError> {
    for music_file in read_music_files()? {
        let (music_file_name, music_file_content) = music_file?;
        music_cache.insert(music_file_name, &music_file_content);
    }
    Ok(())
}

fn read_music_files() -> Result<FolderFileIterator, AudioError> {
    Ok(read_files_in_folder("./music")?)
}