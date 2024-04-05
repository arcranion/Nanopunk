use bevy::app::{App, Startup};
use bevy::prelude::{Commands, Plugin};

use crate::plugins::screen::load::LoadScreenPlugin;

mod load;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins here
        app
            .add_systems(Startup, init_camera)
            .add_plugins((
                LoadScreenPlugin
            ));

        return;
    }
}

fn init_camera(mut commands: Commands) {
    // Add Camera2dBundle for UI
    // commands.spawn(Camera2dBundle::default());
}