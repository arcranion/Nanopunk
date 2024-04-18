use bevy::prelude::*;

use crate::plugins::screen::main::MainScreenPlugin;

mod main;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins here
        app
            .add_plugins((
                MainScreenPlugin
            ));

        return;
    }
}