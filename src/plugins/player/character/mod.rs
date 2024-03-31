use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use crate::plugins::player::character::systems::create_scene;

pub mod components;
mod systems;

pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, create_scene);

        return;
    }
}