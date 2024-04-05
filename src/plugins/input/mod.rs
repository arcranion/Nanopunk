use bevy::app::{App, Plugin, Startup};
use bevy::input::InputSystem;
use bevy::prelude::{Commands, Entity, IntoSystemConfigs, Query, Update, With};
use bevy::window::PrimaryWindow;
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::plugin::{InputManagerPlugin, InputManagerSystem};
use leafwing_input_manager::prelude::ActionStateDriver;
use leafwing_input_manager::systems::run_if_enabled;

use actions::Actions;

use crate::plugins::input::drivers::update_cursor_position;

pub mod actions;
mod drivers;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Actions>::default())
            .add_systems(Startup, create_input_manager);
            // .add_systems(
            //     Update,
            //     update_cursor_position
            //         .in_set(InputManagerSystem::ManualControl)
            // );

        add_driver_system(app, update_cursor_position);

        return;
    }
}

fn add_driver_system<M>(app: &mut App, systems: impl IntoSystemConfigs<M>) {
    app.add_systems(
        Update,
        systems
            .run_if(run_if_enabled::<Actions>)
            .in_set(InputManagerSystem::ManualControl)
            .before(InputManagerSystem::ReleaseOnDisable)
            .after(InputManagerSystem::Tick)
            .after(InputManagerSystem::Update)
            .after(InputSystem),
    );
}

fn create_input_manager(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>
) {
    let input_map = Actions::default_input_map();
    let bundle = InputManagerBundle::with_map(input_map);

    let input_entity = commands
        .spawn(bundle)
        .id();

    commands
        .entity(window.single())
        .insert(ActionStateDriver {
            action: Actions::PlayerLook,
            targets: input_entity.into()
        });
}