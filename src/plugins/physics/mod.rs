use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

        return;
    }
}