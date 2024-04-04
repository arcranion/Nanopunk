use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerInputState {
    pub sprint: bool,
    pub crouch: bool,
    pub jump: bool,
    pub movement: Vec3,
    pub pointer: Vec2,
}

impl Default for PlayerInputState {
    fn default() -> Self {
        return Self {
            sprint: false,
            jump: false,
            crouch: false,
            movement: Default::default(),
            pointer: Vec2::ZERO
        }
    }
}