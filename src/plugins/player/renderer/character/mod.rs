use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerCharacterModelRendererBundle {
    #[bundle()]
    pub scene_bundle: DynamicSceneBundle,
}