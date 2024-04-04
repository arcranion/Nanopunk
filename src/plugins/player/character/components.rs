use bevy::asset::Handle;
use bevy::prelude::{Bundle, Component, Scene};
use bevy::scene::SceneBundle;

#[derive(Bundle)]
pub struct PlayerRendererBundle {
    #[bundle()]
    pub model_scene: SceneBundle
}

impl Default for PlayerRendererBundle {
    fn default() -> Self {
        return Self {
            model_scene: SceneBundle::default()
        }
    }
}