use bevy::app::App;
use bevy::prelude::Plugin;
use crate::plugins::player::movement::PlayerMovementPlugin;

pub mod movement;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin);
    }
}