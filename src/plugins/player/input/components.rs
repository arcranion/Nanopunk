use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerInputState {
    pub sprint: bool,
    pub crouch: bool,
    pub jump: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec3,
}

impl Default for PlayerInputState {
    fn default() -> Self {
        return Self {
            sprint: false,
            jump: false,
            crouch: false,
            pitch: 0.0,
            yaw: 0.0,
            movement: Default::default(),
        }
    }
}