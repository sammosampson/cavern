
use crate::prelude::*;

const LEVEL_1: [&str; 17] = [
    "XXXXX     XXXXXXXX     XXXXX",
    "","","","",
    "   XXXXXXX        XXXXXXX   ",
    "","","",
    "   XXXXXXXXXXXXXXXXXXXXXX   ",
    "","","",
    "XXXXXXXXX          XXXXXXXXX",
    "","",""
];

const LEVEL_2: [&str; 17] = [
    "XXXX    XXXXXXXXXXXX    XXXX",
    "","","","",
    "    XXXXXXXXXXXXXXXXXXXX    ",
    "","","",
    "XXXXXX                XXXXXX",
    "      X              X      ",
    "       X            X       ",
    "        X          X        ",
    "         X        X         ",
    "","",""
];

const LEVEL_3: [&str; 17] = [
    "XXXX    XXXX    XXXX    XXXX",
    "","","","",
    "  XXXXXXXX        XXXXXXXX  ",
    "","","",
    "XXXX      XXXXXXXX      XXXX",
    "","","",
    "    XXXXXX        XXXXXX    ",
    "","",""
];
const LEVELS : [[&str; 17]; 3] = [LEVEL_1, LEVEL_2, LEVEL_3];

const TOTAL_ROWS: usize = 17;
const LEVEL_X_OFFSET: f32 = 50.0;
const GRID_BLOCK_SIZE: f32 = 25.0;
 
pub fn add_level_background(buffer: &mut CommandBuffer) {
    buffer.push((
        Texture::png("bg0"), 
        Layer(0), 
        Position(centre_screen()), 
        WorldEntityId::from("Level")
    ));
}

pub fn add_level_background_block(buffer: &mut CommandBuffer, position: Vector) {
    buffer.push((
        Texture::png("block0"), 
        Layer(1), 
        Position(position), 
        WorldEntityId::from(format!("Block{:?}{:?}", position.x, position.y))
    ));
}

pub fn draw_level(buffer: &mut CommandBuffer, level: usize) {
    let grid = LEVELS[level];

    for row_index in 0..TOTAL_ROWS {
        let row = grid[row_index];
        if row.len() == 0 {
            continue;
        }
    
        let mut x = LEVEL_X_OFFSET;
    
        for block in row.chars() {
            if block != ' ' {
                add_level_background_block(buffer, Vector::new(x, row_index as f32 * GRID_BLOCK_SIZE));
            }
            x += GRID_BLOCK_SIZE;
        }
    }
    
}