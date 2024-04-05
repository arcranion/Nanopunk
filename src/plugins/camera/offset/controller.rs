use bevy::prelude::{FloatExt, Query, Res, Time, Transform, With, Without};
use leafwing_input_manager::prelude::ActionState;

use crate::plugins::camera::offset::components::{OffsetCameraControllerControl, OffsetCameraControllerEntity, OffsetCameraControllerOptions, OffsetCameraControllerState, OffsetCameraControllerTarget};
use crate::plugins::input::actions::Actions;

pub(in super::super) fn offset_controller_control(
    mut query_camera: Query<
        (
            &OffsetCameraControllerControl,
            &mut OffsetCameraControllerState
        ),
        With<OffsetCameraControllerEntity>
    >,
    query_action_state: Query<&ActionState<Actions>>
) {
    let action_state = query_action_state.single();

    for (
        control,
        mut state
    ) in query_camera.iter_mut() {
        let value = action_state.clamped_value(&control.zoom_action);

        state.zoom_target += value * control.zoom_sensitivity;
    }
}

pub(in super::super) fn offset_controller(
    mut query_camera: Query<
        (
            &mut Transform,
            &OffsetCameraControllerTarget,
            &OffsetCameraControllerOptions,
            &mut OffsetCameraControllerState
        ),
        (
            With<OffsetCameraControllerEntity>,
        )
    >,
    query_target: Query<&mut Transform, Without<OffsetCameraControllerEntity>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();

    for (
        mut transform,
        target,
        options,
        mut state
    ) in query_camera.iter_mut() {
        let target = match target {
            OffsetCameraControllerTarget::TargetEntity(entity) =>
                query_target.get(*entity).unwrap().translation,
            OffsetCameraControllerTarget::TargetVector(vector) =>
                *vector
        };

        state.zoom_internal = state.zoom_internal.lerp(state.zoom_target, delta * options.zoom_interpolation_factor);

        let zoom_vector = state.zoom_internal * options.zoom_factor;
        let translation = target + options.offset + zoom_vector;

        transform.translation = transform.translation.lerp(translation, delta * options.translation_interpolation_factor);
    }
}
