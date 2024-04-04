use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Component, Transform};
use bevy_rapier3d::prelude::{Collider, Velocity};

#[derive(Component)]
pub struct PlayerPhysicsEntity;

#[derive(Component)]
pub struct PlayerPhysicsOptions {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub crouched_speed: f32,

    pub jump_force: f32,

    pub direction_factor: Vec3,

    pub air_speed_cap: f32,
    pub air_acceleration: f32,
    pub max_air_speed: f32,

    pub acceleration: f32,
    pub friction: f32,

    /// If the dot product (alignment) of the normal of the surface and the upward vector,
    /// which is a value from [-1, 1], is greater than this value, ground movement is applied
    pub traction_normal_cutoff: f32,
    pub friction_speed_cutoff: f32,

    pub crouch_transition_speed: f32,
    pub uncrouch_transition_speed: f32,

    pub height: f32,
    pub upright_height: f32,
    pub crouch_height: f32,
    pub stop_speed: f32,
    pub step_offset: f32,

    pub pointer_distance_max: f32,

    pub orientation_rotation: Quat
}

impl Default for PlayerPhysicsOptions {
    fn default() -> Self {
        return Self {
            walk_speed: 9.0,
            run_speed: 14.0,
            jump_force: 8.5,
            crouched_speed: 5.0,

            direction_factor: Vec3::new(30.0, 1.0, -30.0),

            air_speed_cap: 2.0,
            air_acceleration: 20.0,
            max_air_speed: 15.0,

            crouch_transition_speed: 6.0,
            uncrouch_transition_speed: 8.0,

            height: 1.5,
            upright_height: 2.0,
            crouch_height: 1.25,

            acceleration: 10.0,
            friction: 10.0,

            traction_normal_cutoff: 0.7,
            friction_speed_cutoff: 0.1,

            stop_speed: 1.0,

            step_offset: 0.0,

            pointer_distance_max: 100.0,

            orientation_rotation: Quat::IDENTITY,
        }
    }
}

#[derive(Component)]
pub struct PlayerPhysicsState {
    pub velocity: Vec3,
    pub target_speed: f32,

    pub ground_time: u8,
}

impl Default for PlayerPhysicsState {
    fn default() -> Self {
        return Self {
            velocity: Vec3::ZERO,
            target_speed: 0.0,

            ground_time: 0,
        }
    }
}