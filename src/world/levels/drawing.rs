use crate::prelude::*;

const LEVEL_X_OFFSET: f32 = 50.0;
const GRID_BLOCK_SIZE: f32 = 25.0;

fn add_level_background_block(buffer: &mut CommandBuffer, position: Vector) {
    buffer.push((
        Texture::png("block0"), 
        Layer(1), 
        Position(position),
        CollisionBox::from(Dimensions::from(GRID_BLOCK_SIZE)),
        StaticRigidBody,
        WorldEntityId::from(format!("Block{:?}{:?}", position.x, position.y))
    ));
}

pub fn draw_level(buffer: &mut CommandBuffer, grid: &LevelGrid) {
    for (row, column) in grid.iter() {
        let x = LEVEL_X_OFFSET + (column as f32 * GRID_BLOCK_SIZE);
        let y = row as f32 * GRID_BLOCK_SIZE;
        add_level_background_block(buffer, Vector::new(x, y));
    }
}
