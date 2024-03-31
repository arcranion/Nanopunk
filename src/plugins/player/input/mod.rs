use bevy::prelude::*;
use crate::plugins::player::input::systems::player_input;

pub mod components;
pub mod systems;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input);

        return;
    }
}