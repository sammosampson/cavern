use crate::prelude::*;

pub struct ScoreBoard(PlayerIndex);


impl Deref for ScoreBoard {
    type Target = PlayerIndex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_player_score() -> PlayerScore {
    PlayerScore::default()
}

#[derive(Default, Debug)]
pub struct PlayerScore {
    inner: [u8;2]
}

impl PlayerScore {
    pub fn game_won(&self) -> bool {
        self.inner[0] == 9 || self.inner[1] == 9
    }

    pub fn increment(&mut self, index: PlayerIndex) {
        self.inner[usize::from(index)] += 1;
    }

    pub fn reset(&mut self) {
        self.inner[0] = 0;
        self.inner[1] = 0;
    }

    pub fn get(&self, index: PlayerIndex) -> u8 {
        self.inner[usize::from(index)]
    }
}

pub fn add_score_sound(buffer: &mut CommandBuffer) {
    buffer.push(create_sound_components(SoundResources::ScoreGoal));   
}

pub fn add_player_one_score(buffer: &mut CommandBuffer) {
    add_score(buffer,TextureResources::Digit1(0), PlayerIndex::Player1, SCREEN_WIDTH - 255.0, "Score1");
}

pub fn add_player_two_score(buffer: &mut CommandBuffer) {
    add_score(buffer,TextureResources::Digit2(0), PlayerIndex::Player2, 255.0, "Score2");
}

fn add_score(buffer: &mut CommandBuffer, texture: TextureResources, index: PlayerIndex, x: f32, name: &str) {
    buffer.push((
        ScoreBoard(index),
        Texture(texture), 
        Layer(2), 
        Position(Vector::new(x, SCREEN_HEIGHT - 46.0)), 
        WorldEntityId::from(name),
    ));
}

pub fn set_standard_score_texture(buffer: &mut CommandBuffer, entity: Entity, score: u8, index: PlayerIndex) {
    if index == PlayerIndex::Player1 {
        set_texture(buffer, entity, TextureResources::Digit1(score));
    } else {
        set_texture(buffer, entity, TextureResources::Digit2(score));
    }
}

pub fn set_lose_score_texture(buffer: &mut CommandBuffer, entity: Entity, score: u8, index: PlayerIndex) {
    if index == PlayerIndex::Player1 {
        set_texture(buffer, entity, TextureResources::Digit0(score));
    } else {
        set_texture(buffer, entity, TextureResources::Digit0(score));
    }
}