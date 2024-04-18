mod systems;

use bevy::app::App;
use bevy::prelude::*;
use crate::plugins::screen::main::systems::update;
use crate::state::app::AppScreenState;

pub struct MainScreenPlugin;

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(AppScreenState::Main)));

        return;
    }
}