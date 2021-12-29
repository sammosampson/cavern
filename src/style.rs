pub fn create_game_style() -> GameStyle {
    GameStyle::OnePlayer
}

#[derive(Clone, Copy)]
pub enum GameStyle {
    OnePlayer,
    TwoPlayer
}
