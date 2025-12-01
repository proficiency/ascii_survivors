use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    FadingIn,
    Game,
    LevelTransition,
    GameOver,
}
