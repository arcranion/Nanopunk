use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use crate::plugins::player::input::systems::player_input;
use crate::plugins::player::renderer::systems::{cache_model, renderer_input, renderer_update};

pub mod components;
mod systems;

pub struct PlayerRendererPlugin;

impl Plugin for PlayerRendererPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                cache_model,
                renderer_input.after(cache_model).after(player_input),
                renderer_update.after(renderer_input)
            ));

        return;
    }
}