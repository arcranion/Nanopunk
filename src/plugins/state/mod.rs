use bevy::app::App;
use bevy::prelude::Plugin;
use crate::state::app::{AppGameState, AppLoadState, AppScreenState};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // Create initial state
        app
            .insert_state(AppLoadState::LoadingBaseAssets)
            .insert_state(AppScreenState::Loading)
            .insert_state(AppGameState::Paused);

        return;
    }
}