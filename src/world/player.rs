use crate::prelude::*;

pub const BAT_POSITION_OFFSET: f32 = 40.0;
pub const BAT_HEIGHT: f32 = 100.0;
pub const BAT_WIDTH: f32 = 18.0;
pub const HALF_BAT_HEIGHT: f32 = BAT_HEIGHT * 0.5;
pub const HALF_BAT_WIDTH: f32 = BAT_WIDTH * 0.5;

pub fn create_player_score() -> PlayerScore {
    PlayerScore::default()
}

#[derive(Default, Debug)]
pub struct PlayerScore {
    player1: u8,
    player2: u8
}

impl PlayerScore {
    pub fn game_won(&self) -> bool {
        self.player1 == 9 || self.player2 == 9
    }

    pub fn increment(&mut self, index: u8) {
        if index == 0 {
            self.player1 += 1;
        } else {
            self.player2 += 1;
        }
    }
}

pub struct Player;

#[derive(Default, Debug, Copy, Clone)]
pub struct Bat(pub u8);

impl Deref for Bat {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn set_bat_score_texture(buffer: &mut CommandBuffer, bat_entity: Entity, index: u8) {
    let texture = if index == 0 { TextureResources::Bat0(2) } else { TextureResources::Bat1(2) };
    set_texture(buffer, bat_entity, texture);
}

pub fn set_normal_bat_texture(buffer: &mut CommandBuffer, bat_entity: Entity, index: u8) {
    let texture = if index == 0 { TextureResources::Bat0(0) } else { TextureResources::Bat1(0) };
    set_texture(buffer, bat_entity, texture);
}

pub fn set_bat_hit_effect(buffer: &mut CommandBuffer, game_timer: &GameTimer, bat_entity: Entity, index: u8) {
    buffer.add_component(bat_entity, create_bat_hit_effect_animation(game_timer, index));
}

fn create_bat_hit_effect_animation(game_timer: &GameTimer, index: u8) -> Animation {
    let mut animation = create_animation(
        Duration::from_secs(1).as_secs_f32(), 
        game_timer.total_game_time());

    if index == 0 {
        animation.add_frame(TextureResources::Bat0(1));
        animation.add_frame(TextureResources::Bat0(0));
    } else {        
        animation.add_frame(TextureResources::Bat1(1));
        animation.add_frame(TextureResources::Bat1(0));
    }

    animation
}


pub fn add_bat0(buffer: &mut CommandBuffer) {
    add_bat(buffer,0,TextureResources::Bat0(0),BAT_POSITION_OFFSET,"Player1");
}

pub fn add_bat1(buffer: &mut CommandBuffer) {
    add_bat(buffer,1,TextureResources::Bat1(0),SCREEN_WIDTH - BAT_POSITION_OFFSET,"Player2");
}

fn add_bat(buffer: &mut CommandBuffer, index: u8, texture: TextureResources, x: f32, name: &str) {
    buffer.push((
        Bat(index),
        Texture(texture), 
        Layer(2), 
        Position(Vector::new(x, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(140.0),
        Heading::default(),
        WorldEntityId::from(name),
        Player
    ));
}

