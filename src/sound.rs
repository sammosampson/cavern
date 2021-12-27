pub fn create_sound_components(resource: SoundResources) -> (Sound, ) {
    (Sound(resource, 1), )
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoundResources {
    Bounce(u8),
    BounceSynth,
    Hit(u8),
    HitSlow,
    HitMedium,
    HitFast,
    HitVeryFast,
    ScoreGoal,
    Up,
    Down
}

#[derive(Debug)]
pub enum SoundError {
}

#[derive(Debug, Copy, Clone)]
pub struct Sound(SoundResources, usize);

pub struct AudioPlayer;

impl AudioPlayer {
    pub fn new() -> Result<Self, SoundError>{
        Ok(Self)
    }

    
    pub fn play_sound(&self, sound: &Sound) -> Result<(), SoundError> {
        println!("Playing sound: {:?}", sound.0);
        //TODO
        Ok(())
    }
}
