use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerInputState {
    pub pointing: Vec2,
    pub movement_normalized: Vec2,

    pub jumping: bool,
    pub crouching: bool,
    pub sprinting: bool
}

impl Default for PlayerInputState {
    fn default() -> Self {
        return Self {
            pointing: Vec2::ZERO,
            movement_normalized: Vec2::ZERO,

            jumping: false,
            crouching: false,
            sprinting: false
        }
    }
}