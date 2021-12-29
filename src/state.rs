#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    None,
    Starting,
    Playing,
    Finishing,
    Exiting
}

pub fn create_game_state() -> GameState {
    GameState::initial()
}

pub struct GameState {
    status: GameStatus,
    previous_status: GameStatus,
    entered_on: Option<f32>
}

impl GameState {
    fn initial() -> Self {
        Self {
            status: GameStatus::Starting,
            previous_status: GameStatus::None,
            entered_on: None
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn previous_status(&self) -> GameStatus {
        self.previous_status
    }

    pub fn transition_to(&mut self, to: GameStatus) {
        self.previous_status = self.status;
        self.status = to;
        self.entered_on = None;
        println!("transitioning to state {:?} from {:?} ", to, self.previous_status);
    }

    pub fn has_entered(&self) -> bool {
        self.entered_on.is_some()
    }

    pub fn enter(&mut self, time: f32) {
        println!("entering state at {:?}", time);
        self.entered_on = Some(time);
    }
}