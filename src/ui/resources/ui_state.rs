#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiState {
    #[default]
    Hud,
    Menu,
    Pause,
}

