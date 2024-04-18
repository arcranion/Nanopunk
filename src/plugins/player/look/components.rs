use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerLookBundle {
    pub options: PlayerLookOptions,
    pub state: PlayerLookState
}

impl Default for PlayerLookBundle {
    fn default() -> Self {
        return Self {
            options: PlayerLookOptions::default(),
            state: PlayerLookState::default()
        }
    }
}

#[derive(Component)]
pub struct PlayerLookOptions {
    pub distance_max: f32
}

impl Default for PlayerLookOptions {
    fn default() -> Self {
        return Self {
            distance_max: 100.0
        }
    }
}

#[derive(Component)]
pub struct PlayerLookState {
    pub target_yaw: f32,
    pub target_pitch: f32,

    pub yaw: f32,
    pub pitch: f32
}

impl Default for PlayerLookState {
    fn default() -> Self {
        return Self {
            target_yaw: 0.0,
            target_pitch: 0.0,

            yaw: 0.0,
            pitch: 0.0
        }
    }
}