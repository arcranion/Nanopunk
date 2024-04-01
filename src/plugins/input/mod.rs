use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::plugin::InputManagerPlugin;
use actions::Actions;

pub mod actions;
mod drivers;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Actions>::default())
            .add_systems(Startup, create_input_manager);

        return;
    }
}

fn create_input_manager(mut commands: Commands) {
    let input_map = Actions::default_input_map();
    let bundle = InputManagerBundle::with_map(input_map);

    commands.spawn(bundle);
}
