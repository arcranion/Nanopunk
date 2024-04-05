use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter};

use crate::plugins::screen::load::systems::enter_screen;
use crate::state::app::AppScreenState;

mod systems;

pub struct LoadScreenPlugin;

impl Plugin for LoadScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppScreenState::Loading), enter_screen);

        return;
    }
}