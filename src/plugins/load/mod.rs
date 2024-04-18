mod systems;

use bevy::prelude::*;
use crate::plugins::load::systems::{AssetLoadBank, queue_assets, update_state};
use crate::state::app::{AppLoadState, AppScreenState};

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AssetLoadBank {
                loads: Vec::new()
            })
            .add_systems(Startup, queue_assets)
            .add_systems(Update, update_state.run_if(in_state(AppScreenState::Loading)));

        return;
    }
}