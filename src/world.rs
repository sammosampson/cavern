use crate::prelude::*;

pub const BOUNDS_MIN_Y: f32 = 32.0;
pub const BOUNDS_MAX_Y: f32 = SCREEN_HEIGHT - BOUNDS_MIN_Y;
pub const BAT_POSITION_OFFSET: f32 = 40.0;
pub const BAT_HEIGHT: f32 = 100.0;
pub const BAT_WIDTH: f32 = 18.0;
pub const HALF_BAT_HEIGHT: f32 = BAT_HEIGHT * 0.5;
pub const HALF_BAT_WIDTH: f32 = BAT_WIDTH * 0.5;
pub const BALL_DIAMETER: f32 = 14.0;
pub const BALL_RADIUS: f32 = BALL_DIAMETER* 0.5;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct WorldEntityId(String);

#[derive(Default, Debug, Copy, Clone)]
pub struct Ball;

#[derive(Default, Debug, Copy, Clone)]
pub struct Bat(pub u8);



impl Deref for Bat {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for WorldEntityId {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for WorldEntityId {
    fn from(name: String) -> Self {
        Self(name)
    }
}

pub struct Player;

pub fn add_items_to_world(world: &mut World) {
    world.push((
        Texture(TextureResources::Table), 
        Layer(0),
        Position(centre_screen()),
        WorldEntityId::from("arena")
    ));

    world.push((
        Ball,
        Texture(TextureResources::Ball), 
        Layer(1), 
        Position(centre_screen()), 
        MaximumVelocity(150.0),
        Heading(Angle::from_degrees(135.0).into()),
        WorldEntityId::from("ball")
    ));

    let player = world.push((
        Bat(0),
        Texture(TextureResources::Bat00), 
        Layer(1), 
        Position(Vector::new(BAT_POSITION_OFFSET, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(140.0),
        Heading::default(),
        WorldEntityId::from("player1")
    ));

    if let Some(mut entry) = world.entry(player) {
        entry.add_component(Player);
    }

    let player = world.push((
        Bat(1),
        Texture(TextureResources::Bat10), 
        Layer(1), 
        Position(Vector::new(SCREEN_WIDTH - BAT_POSITION_OFFSET, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(140.0),
        Heading::default(),
        WorldEntityId::from("player2")
    ));

    if let Some(mut entry) = world.entry(player) {
        entry.add_component(Player);
    }
}

fn centre_screen() -> Vector {
    Vector::new(HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT)
}