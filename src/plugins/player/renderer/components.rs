use bevy::prelude::{Bundle, Component, Entity};
use bevy::scene::SceneBundle;

#[derive(Bundle)]
pub struct PlayerRendererBundle {
    #[bundle()]
    pub model_scene: SceneBundle,

    pub options: PlayerRendererOptions,
    pub state: PlayerRendererState,
    pub cache: PlayerRendererCache,
}

impl Default for PlayerRendererBundle {
    fn default() -> Self {
        return Self {
            model_scene: SceneBundle::default(),

            options: PlayerRendererOptions::default(),
            state: PlayerRendererState::default(),
            cache: PlayerRendererCache::default()
        }
    }
}

#[derive(Component)]
pub struct PlayerRendererOptions {
    pub head_name: String
}

impl Default for PlayerRendererOptions {
    fn default() -> Self {
        return Self {
            head_name: String::new()
        }
    }
}

#[derive(Component)]
pub struct PlayerRendererState {
    pub target_yaw: f32,
    pub target_pitch: f32,

    pub yaw: f32,
    pub pitch: f32,
}

impl Default for PlayerRendererState {
    fn default() -> Self {
        return Self {
            target_yaw: 0.0,
            target_pitch: 0.0,

            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerRendererCache {
    pub cached_head: Entity
}

impl Default for PlayerRendererCache {
    fn default() -> Self {
        return Self {
            cached_head: Entity::PLACEHOLDER
        }
    }
}

#[derive(Component)]
pub struct PlayerRendererCached;