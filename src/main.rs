
mod application;
mod input;
mod events;
mod systems;
mod rendering;
mod world;
mod state;
mod time;
mod math;
mod geometry;
mod movement;

mod prelude {
    pub use std::io::Cursor;
    pub use std::ops::*;
    pub use std::collections::*;
    pub use crate::application::*;
    pub use crate::input::*;
    pub use crate::events::*;
    pub use crate::systems::*;
    pub use crate::rendering::*;
    pub use crate::world::*;
    pub use crate::state::*;
    pub use crate::time::*;
    pub use crate::math::*;
    pub use crate::geometry::*;
    pub use crate::movement::*;
}

use prelude::*;

fn main() {
    Application::build()
        .expect("Application did not build")
        .run();
}