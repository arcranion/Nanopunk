use bevy::prelude::{Bundle, Component, Entity, GlobalTransform, InheritedVisibility, Transform, ViewVisibility, Visibility};
use bevy::scene::SceneBundle;
use crate::plugins::player::renderer::character::PlayerCharacterModelRendererBundle;
use crate::plugins::player::renderer::weapon::PlayerWeaponModelRendererBundle;

#[derive(Bundle, Default)]
pub struct PlayerRendererBundle {
    pub entity: PlayerRendererEntity,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility
}

#[derive(Component, Default)]
pub struct PlayerRendererEntity;