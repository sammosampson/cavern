
mod application;
mod input;
mod audio;
mod events;
mod systems;
mod rendering;
mod physics;
mod world;
mod state;
mod time;
mod math;
mod geometry;
mod movement;
mod animation;
mod files;

mod prelude {
    pub use std::io::Cursor;
    pub use std::ops::*;
    pub use std::collections::*;
    pub use smallvec::SmallVec;
    pub use std::time::Duration;
    pub use itertools::Itertools;
    pub use crate::application::*;
    pub use crate::input::*;
    pub use crate::audio::*;
    pub use crate::events::*;
    pub use crate::systems::*;
    pub use crate::rendering::*;
    pub use crate::physics::*;
    pub use crate::world::*;
    pub use crate::state::*;
    pub use crate::time::*;
    pub use crate::math::*;
    pub use crate::geometry::*;
    pub use crate::movement::*;
    pub use crate::animation::*;
    pub use crate::files::*;
}

use prelude::*;

fn main() {
    Application::build()
        .expect("Application did not build")
        .run();
}