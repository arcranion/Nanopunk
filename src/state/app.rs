use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppLoadState {
    LoadingBaseAssets, // To load basic assets like loading screen ui
    LoadingInitialAssets, // Load other big files
    Loaded // Loaded
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppScreenState {
    Loading, // Loading UI
    Main, // Main UI
    Game // In-game
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppGameState {
    Playing,
    Paused
}