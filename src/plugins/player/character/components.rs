use bevy::asset::Handle;
use bevy::prelude::{Bundle, Component, Scene};
use bevy::scene::SceneBundle;

#[derive(Bundle)]
pub struct PlayerCharacterBundle {
    #[bundle()]
    pub scene: SceneBundle
}

impl Default for PlayerCharacterBundle {
    fn default() -> Self {
        return Self {
            scene: SceneBundle::default()
        }
    }
}