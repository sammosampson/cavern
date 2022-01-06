use crate::prelude::*;

#[derive(Debug)]
pub enum ApplicationError {
    RendererError(RendererError),
    TextureError(TextureError),
    AudioError(AudioError),
}

impl From<AudioError> for ApplicationError {
    fn from(error: AudioError) -> Self {
        Self::AudioError(error)
    }
}

pub struct Application {
    world: World, 
    resources: Resources,
    start_schedule: Schedule,
    play_schedule: Schedule,
    finish_schedule: Schedule,
    event_loop: SystemEventLoop
}

impl Application {
    pub fn build() -> Result<Self, ApplicationError> {
        let event_loop = create_system_event_loop();
        let world = build_world();
        let resources = build_resources(&event_loop)?;
        let start_schedule = build_start_schedule();
        let play_schedule = build_play_schedule();
        let finish_schedule = build_finish_schedule();
       
        let application = Self {
            world,
            resources, 
            start_schedule,
            play_schedule,
            finish_schedule,
            event_loop
        };

        Ok(application)
    }

    pub fn run(&mut self) {
        loop {
            if !self.run_loop() {
                return;
            }
        }
    }
    
    fn run_loop(&mut self) -> bool {
        self.process_events();
        self.execute_schedule()     
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<SystemEventChannel>().unwrap();
        let mut screen_renderer = &mut self.resources.get_mut::<ScreenRenderer>().unwrap();
        self.event_loop.run(&mut event_producer, &mut event_channel, &mut screen_renderer);
    
    }

    fn execute_schedule(&mut self) -> bool {
        let current_state = self.resources.get::<GameState>().unwrap().status();
        
        match current_state {
            GameStatus::None => {},
            GameStatus::Starting => self.start_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Playing => self.play_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Finishing => self.finish_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Exiting => return false
        };

        true
    }
}

fn build_resources(event_loop: &SystemEventLoop) -> Result<Resources, ApplicationError> {
    let screen_renderer = create_screen_renderer(event_loop)?;
    let editor_renderer = create_editor_renderer(&screen_renderer.display);
    let texture_cache = create_texture_cache(&screen_renderer)?;
    let game_timer = create_game_timer();
    let game_state = create_game_state();
    let editor_graph = create_editor_graph();
    let system_event_producer = create_system_event_producer();
    let system_event_channel = create_system_event_channel();
    let audio = create_audio_player();
    let sound_cache = create_sound_cache()?;
    let music_cache = create_music_cache()?;
        
    let mut resources = Resources::default();
    resources.insert(screen_renderer);
    resources.insert(editor_renderer);
    resources.insert(texture_cache);
    resources.insert(game_timer);
    resources.insert(system_event_producer);
    resources.insert(system_event_channel);
    resources.insert(game_state);
    resources.insert(editor_graph);
    resources.insert(audio);
    resources.insert(sound_cache);
    resources.insert(music_cache);
    Ok(resources)
}

pub fn create_texture_cache(screen_renderer: &ScreenRenderer) -> Result<TextureCache, ApplicationError> {
    let mut textures = TextureCache::default();

    initialise_texture_cache(&mut textures, screen_renderer)
        .map_err(|error| ApplicationError::TextureError(error))?;

    Ok(textures)
}

fn create_screen_renderer(event_loop: &SystemEventLoop) -> Result<ScreenRenderer, ApplicationError> {
    Ok(
        ScreenRenderer::new(&event_loop.get_loop())
            .map_err(|error| ApplicationError::RendererError(error))?
    )
}

fn create_sound_cache() -> Result<SoundSourceCache, ApplicationError> {
    let mut sounds = SoundSourceCache::default();
    initialise_sound_cache(&mut sounds)?;
    Ok(sounds)
}

fn create_music_cache() -> Result<MusicSourceCache, ApplicationError> {
    let mut music = MusicSourceCache::default();
    initialise_music_cache(&mut music)?;
    Ok(music)
}