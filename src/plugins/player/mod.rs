use bevy::app::App;
use bevy::prelude::Plugin;
use crate::plugins::player::input::PlayerInputPlugin;
use crate::plugins::player::physics::PlayerPhysicsPlugin;
use crate::plugins::player::renderer::PlayerRendererPlugin;

pub mod physics;
pub mod input;
pub mod components;
pub mod inventory;
pub mod tool;
pub mod bundle;
pub mod health;
pub mod renderer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPhysicsPlugin,
            PlayerInputPlugin,
            PlayerRendererPlugin
        ));
    }
}