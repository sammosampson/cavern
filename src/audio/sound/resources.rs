use crate::prelude::*;

#[derive(Default)]
pub struct SoundSourceCache {
    inner: HashMap<String, RawSoundSource>
}

impl SoundSourceCache {
    fn insert(&mut self, name: String, data: &[u8]) {
        self.inner.insert(name, RawSoundSource { bytes: data.to_vec() });
    }

    pub fn get(&self, resource: &str) -> Option<&RawSoundSource> {
        self.inner.get(resource)
    }
}

pub fn initialise_sound_cache(sound_cache: &mut SoundSourceCache) -> Result<(), AudioError> {
    for sound_file in read_sound_files()? {
        let (sound_file_name, sound_file_content) = sound_file?;
        sound_cache.insert(sound_file_name, &sound_file_content);
    }
    Ok(())
}

fn read_sound_files() -> Result<FolderFileIterator, AudioError> {
    Ok(read_files_in_folder("./sounds")?)
}