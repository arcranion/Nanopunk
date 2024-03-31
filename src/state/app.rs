use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppLoadState {
    #[default]
    LoadingBaseAssets, // To load basic assets like loading screen ui
    LoadingInitialAssets, // Load other big files
    Loaded // Loaded
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppScreenState {
    #[default]
    Loading, // Loading UI
    Main, // Main UI
    Game // In-game
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppGameState {
    #[default]
    Playing,
    Paused
}