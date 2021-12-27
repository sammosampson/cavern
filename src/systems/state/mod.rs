mod exiting;
mod starting;
mod playing;
mod scoring;
mod finishing;

pub use exiting::*;
pub use starting::*;
pub use playing::*;
pub use scoring::*;
pub use finishing::*;

use crate::prelude::*;

fn set_normal_bat_textures(buffer: &mut CommandBuffer, world: &SubWorld) {
    <(Entity, &Bat)>::query()
        .iter(world)
        .for_each(|(entity, bat)|{
            set_normal_bat_texture(buffer, *entity, **bat);
        });
}