mod instances;
pub use instances::*;

use crate::prelude::*;

pub struct RenderItem {
    pub entity_id: WorldEntityId,
    pub layer: u8,
    pub texture: String,
    pub centre_position: Vector
}

impl From<(&WorldEntityId, &Texture, &Position, &Layer)> for RenderItem {
    fn from(from: (&WorldEntityId, &Texture, &Position, &Layer)) -> Self {
        let (id, texture, position, layer) = from;
        Self {
            entity_id: id.clone(),
            layer: **layer,
            texture: (**texture).to_string(),
            centre_position: **position
        }
    }
}
