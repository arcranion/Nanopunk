use bevy::prelude::*;

use crate::plugins::player::input::systems::player_input;
use crate::state::app::AppScreenState;

pub mod components;
pub mod bundle;
pub mod systems;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            systems::player_update_speed.after(player_input),
            systems::player_movement.after(systems::player_update_speed)
        ).run_if(in_state(AppScreenState::Game)));

        return;
    }
}