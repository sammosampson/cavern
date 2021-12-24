use crate::prelude::*;

pub const BAT_POSITION_OFFSET: f32 = 40.0;
pub const BAT_HEIGHT: f32 = 100.0;
pub const BAT_WIDTH: f32 = 18.0;
pub const HALF_BAT_HEIGHT: f32 = BAT_HEIGHT * 0.5;
pub const HALF_BAT_WIDTH: f32 = BAT_WIDTH * 0.5;

pub struct Player;

#[derive(Default, Debug, Copy, Clone)]
pub struct Bat(pub u8);

impl Deref for Bat {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn add_bat0(buffer: &mut CommandBuffer) {
    add_bat(buffer,0,TextureResources::Bat00,BAT_POSITION_OFFSET,"Player1");
}

pub fn add_bat1(buffer: &mut CommandBuffer) {
    add_bat(buffer,1,TextureResources::Bat10,SCREEN_WIDTH - BAT_POSITION_OFFSET,"Player2");
}

fn add_bat(buffer: &mut CommandBuffer, index: u8, texture: TextureResources, x: f32, name: &str) {
    buffer.push((
        Bat(index),
        Texture(texture), 
        Layer(1), 
        Position(Vector::new(x, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(140.0),
        Heading::default(),
        WorldEntityId::from(name),
        Player
    ));
}

