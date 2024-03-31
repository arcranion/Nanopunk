use bevy::app::App;
use bevy::prelude::Plugin;
use crate::plugins::player::physics::PlayerPhysicsPlugin;

pub mod physics;
pub mod input;
pub mod components;
pub mod inventory;
pub mod tool;
pub mod bundle;
pub mod health;
pub mod character;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPhysicsPlugin);
    }
}