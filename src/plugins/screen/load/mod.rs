mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Update};
use crate::plugins::screen::load::systems::{enter_screen};
use crate::state::app::{AppLoadState, AppScreenState};

pub struct LoadScreenPlugin;

impl Plugin for LoadScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppScreenState::Loading), enter_screen);

        return;
    }
}