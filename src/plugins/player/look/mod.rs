use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use crate::plugins::player::input::systems::player_input;
use crate::plugins::player::look::systems::{look_input, look_update};

pub mod components;
pub mod systems;

pub struct PlayerLookPlugin;

impl Plugin for PlayerLookPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                look_input.after(player_input),
                look_update.after(look_input)
            ));

        return;
    }
}