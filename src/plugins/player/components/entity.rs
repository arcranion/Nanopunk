use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerEntity;

#[derive(Component)]
pub struct PlayerOptions {
    pub speed_walking: f32,
    pub speed_crouching: f32,
    pub speed_sprinting: f32,

    pub force_jump: f32,

    pub friction_factor: f32,
    pub midair_factor: f32,
    pub speed_interpolation_factor: f32,

    pub orientation_rotation: Quat
}

impl Default for PlayerOptions {
    fn default() -> Self {
        return Self {
            speed_walking: 5.0,
            speed_crouching: 3.0,
            speed_sprinting: 7.5,
            force_jump: 5.0,

            friction_factor: 15.0,
            midair_factor: 1.0,
            speed_interpolation_factor: 10.0,

            orientation_rotation: Quat::IDENTITY
        }
    }
}

#[derive(Component)]
pub struct PlayerInputState {
    pub movement_normalized: Vec2,

    pub jumping: bool,
    pub crouching: bool,
    pub sprinting: bool
}

impl Default for PlayerInputState {
    fn default() -> Self {
        return Self {
            movement_normalized: Vec2::ZERO,

            jumping: false,
            crouching: false,
            sprinting: false
        }
    }
}

#[derive(Component)]
pub struct PlayerPhysics {
    pub velocity: Vec3,
    pub speed: f32
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        return Self {
            velocity: Vec3::ZERO,
            speed: 0.0
        }
    }
}