use bevy::prelude::*;
use crate::plugins::player::input::systems::player_input;
use crate::state::app::AppScreenState;

pub mod components;
pub mod systems;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_input
                .run_if(in_state(AppScreenState::Game))
        ));

        return;
    }
}