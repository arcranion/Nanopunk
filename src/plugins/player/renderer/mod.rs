use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};
use crate::plugins::player::input::systems::player_input;
use crate::plugins::player::look::systems::look_input;
use crate::state::app::AppScreenState;

pub mod components;
mod systems;
mod character;
mod weapon;

pub struct PlayerRendererPlugin;

impl Plugin for PlayerRendererPlugin {
    fn build(&self, app: &mut App) {
        // app
            // .add_systems(Update, ().run_if(in_state(AppScreenState::Game)));

        return;
    }
}