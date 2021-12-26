use crate::prelude::*;

#[derive(Debug)]
pub enum ApplicationError {
    RendererError(RendererError),
    TextureError(TextureError),
    SoundError(SoundError)
}
pub struct Application {
    pub world: World, 
    pub resources: Resources,
    start_schedule: Schedule,
    play_schedule: Schedule,
    score_schedule: Schedule,
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
        let score_schedule = build_score_schedule();
        let finish_schedule = build_finish_schedule();
       
        let application = Self {
            world,
            resources, 
            start_schedule,
            play_schedule,
            score_schedule,
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
        self.event_loop.run(&mut event_producer, &mut event_channel);
    
    }

    fn execute_schedule(&mut self) -> bool {
        let current_state = self.resources.get::<GameState>().unwrap().status();
        match current_state {
            GameStatus::None => {},
            GameStatus::Starting => self.start_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Playing => self.play_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Scoring(_) => self.score_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Finishing => self.finish_schedule.execute(&mut self.world, &mut self.resources),
            GameStatus::Exiting => return false
        };        
        true
    }
}

fn build_resources(event_loop: &SystemEventLoop) -> Result<Resources, ApplicationError> {
    let game_style = create_game_style();
    let screen_renderer = create_screen_renderer(event_loop)?;
    let texture_cache = create_texture_cache(&screen_renderer)?;
    let item_renderer = create_item_renderer();
    let player_score = create_player_score();
    let game_timer = create_game_timer();
    let game_state = create_game_state();
    let system_event_producer = create_system_event_producer();
    let system_event_channel = create_system_event_channel();
    let audio_player = create_audio_player()?;
        
    let mut resources = Resources::default();
    &mut resources.insert(game_style);
    &mut resources.insert(screen_renderer);
    &mut resources.insert(texture_cache);
    &mut resources.insert(item_renderer);
    &mut resources.insert(game_timer);
    &mut resources.insert(player_score);
    &mut resources.insert(system_event_producer);
    &mut resources.insert(system_event_channel);
    &mut resources.insert(audio_player);
    &mut resources.insert(game_state);
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

fn create_audio_player() -> Result<AudioPlayer, ApplicationError> {
    Ok(
        AudioPlayer::new().map_err(|error| ApplicationError::SoundError(error))?
    )
}