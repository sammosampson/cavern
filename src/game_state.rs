#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing
}

pub fn create_exit_state_notifier() -> ExitStateNotifier {
    ExitStateNotifier::default()
}

#[derive(Default)]
pub struct ExitStateNotifier {
    pub should_exit: bool
}