use std::ops::{Neg, Range};
use std::os::linux::raw::stat;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Bundle, Camera3dBundle, Component, Entity, FloatExt, Query, Res, Time, Transform, With, Without};
use leafwing_input_manager::prelude::ActionState;
use crate::plugins::input::actions::Actions;

#[derive(Bundle)]
pub struct Orbit3dCameraControllerBundle {
    pub entity: Orbit3dCameraControllerEntity,
    pub options: Orbit3dCameraControllerOptions,
    pub target: Orbit3dCameraControllerTarget,
    pub state: Orbit3dCameraControllerState
}

#[derive(Component)]
pub struct Orbit3dCameraControllerEntity;

#[derive(Component)]
pub struct Orbit3dCameraControllerOptions {
    pub sensitivity: f32,
    pub distance_range: Range<f32>,
    pub distance_sensitivity: f32,
    pub interpolation_factor: f32
}

#[derive(Component)]
pub enum Orbit3dCameraControllerTarget {
    OriginEntity(Entity),
    OriginVector(Vec3)
}

#[derive(Component)]
pub struct Orbit3dCameraControllerState {
    pub yaw: f32,
    pub pitch: f32,
    pub height: f32,

    pub yaw_target: f32,
    pub pitch_target: f32,
    pub height_target: f32
}

pub(super) fn orbit3d_controller(
    mut query_camera: Query<
        (
            &mut Transform,
            &Orbit3dCameraControllerTarget,
            &mut Orbit3dCameraControllerState,
            &Orbit3dCameraControllerOptions
        ), With<Orbit3dCameraControllerEntity>>,
    query_action_state: Query<&ActionState<Actions>>,
    query_target: Query<(&mut Transform), Without<Orbit3dCameraControllerEntity>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();

    let action_state = query_action_state.single();

    for (mut transform, target, mut state, options) in query_camera.iter_mut() {
        let target_vector = match target {
            Orbit3dCameraControllerTarget::OriginEntity(entity) =>
                query_target.get(*entity).unwrap().translation,
            Orbit3dCameraControllerTarget::OriginVector(vector) =>
                *vector
        };


        state.yaw = state.yaw.lerp(state.yaw_target, delta * options.interpolation_factor);
        state.pitch = state.pitch.lerp(state.pitch_target, delta * options.interpolation_factor);
        state.height = state.height.lerp(state.height_target, delta * options.interpolation_factor);

        if action_state.pressed(&Actions::DevActiveOrbit) {
            let orbit_delta = action_state.axis_pair(&Actions::DevOrbit3d).unwrap();
            let distance_delta = action_state.value(&Actions::DevZoomControl);
            let orbit_normalized = orbit_delta.xy() * options.sensitivity * delta;

            state.pitch_target += orbit_normalized.y;
            state.yaw_target -= orbit_normalized.x;
            state.height_target += -distance_delta * options.distance_sensitivity;
        }

        transform.rotation = Quat::from_rotation_y(state.yaw) * Quat::from_rotation_x(-state.pitch);
        transform.translation = target_vector + transform.rotation * Vec3::new(0.0, 0.0, state.height);
    }
}