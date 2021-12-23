
mod rendering;
mod math;
mod events;
mod application;
mod systems;
mod game_state;
mod geometry;
mod movement;
mod world;
mod time;
mod input;
mod collision;
mod animation;
mod sound;

mod prelude {
    pub use std::io::Cursor;
    pub use std::ops::*;
    pub use std::collections::*;
    pub use std::time::Instant;
    pub use core::ops::*;
    pub use core::time::*;
    pub use smallvec::SmallVec;
    pub use crate::math::*;
    pub use crate::geometry::*;
    pub use crate::events::*;
    pub use crate::rendering::*;
    pub use crate::application::*;
    pub use crate::systems::*;
    pub use crate::game_state::*;
    pub use crate::movement::*;
    pub use crate::world::*;
    pub use crate::time::*;
    pub use crate::input::*;
    pub use crate::collision::*;
    pub use crate::animation::*;
    pub use crate::sound::*;
}

use prelude::*;

fn main() {
    Application::build()
    .expect("Application did not build")
    .run();
}