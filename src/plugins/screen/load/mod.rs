use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, OnExit};

use crate::plugins::screen::load::systems::{enter_screen, exit_screen};
use crate::state::app::AppScreenState;

mod systems;

pub struct LoadScreenPlugin;

impl Plugin for LoadScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppScreenState::Loading), enter_screen)
            .add_systems(OnExit(AppScreenState::Loading), exit_screen);

        return;
    }
}