use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerWeaponModelRendererBundle {
    #[bundle()]
    pub scene_bundle: DynamicSceneBundle,
}