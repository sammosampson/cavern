use std::time::Duration;

use crate::prelude::*;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct ElapsedTime(pub f32);

pub fn create_game_timer() -> GameTimer {
    GameTimer::new()
}

pub struct GameTimer {
    time: Instant,
    last_frame_time: Duration,
    total_game_time: Duration,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            last_frame_time: Duration::default(),
            total_game_time: Duration::default(),
        }
    }

    pub fn last_frame_time(&self) -> f32 {
        self.last_frame_time.as_secs_f32()
    }

    pub fn total_game_time(&self) -> f32 {
        self.total_game_time.as_secs_f32()
    }

    pub fn mark_frame(&mut self) {
        let now = Instant::now();
        self.last_frame_time = now - self.time;
        self.total_game_time += self.last_frame_time;
        self.time = now;
    }
}
