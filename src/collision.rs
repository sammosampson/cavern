use crate::prelude::*;

pub fn create_wall_collision() -> Collision {
    Collision(CollidedWith::Wall)
}

pub fn create_bat_collision() -> Collision {
    Collision(CollidedWith::Bat)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollidedWith {
    Wall,
    Bat
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Collision(CollidedWith);

impl Deref for Collision {
    type Target = CollidedWith;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_impact(game_timer: &GameTimer, position: Position) -> (WorldEntityId, Animation, Position) {
 (create_impact_id(game_timer), create_impact_animation(game_timer), position)
}

fn create_impact_id(game_timer: &GameTimer) -> WorldEntityId {
    WorldEntityId::from(format!("impact{:?})", game_timer.total_game_time()))
}

fn create_impact_animation(game_timer: &GameTimer) -> Animation {
    let mut animation = create_animation(
        Duration::from_millis(333).as_secs_f32(), 
        game_timer.total_game_time());

    animation.add_frame(TextureResources::Impact0);
    animation.add_frame(TextureResources::Impact1);
    animation.add_frame(TextureResources::Impact2);
    animation.add_frame(TextureResources::Impact3);
    animation.add_frame(TextureResources::Impact4);

    animation
}
