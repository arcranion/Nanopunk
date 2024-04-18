use bevy::math::Quat;
use bevy::prelude::{Camera, Color, Gizmos, GlobalTransform, Plane3d, Query, Transform, With};
use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::input::components::PlayerInputState;
use crate::plugins::player::look::components::{PlayerLookOptions, PlayerLookState};

pub(crate) fn look_input(
    mut query_player: Query<
        (
            &PlayerInputState,
            &PlayerLookOptions,
            &mut PlayerLookState,
            &Transform
        ), With<PlayerEntity>
    >,
    query_camera: Query<(&Camera, &GlobalTransform)>,

    mut gizmos: Gizmos
) {
    let (camera, camera_global_transform) = query_camera.single();

    for (
        input_state,
        look_options,
        mut look_state,
        transform
    ) in query_player.iter_mut() {
        let point = camera
            .viewport_to_world(camera_global_transform, input_state.pointer)
            .map(|ray| {
                let distance = if let Some(distance) = ray.intersect_plane(transform.translation, Plane3d::default()) {
                    distance
                } else {
                    look_options.distance_max
                };

                return ray.get_point(distance);
            });

        if let Some(point) = point {
            gizmos.sphere(point, Quat::IDENTITY, 2.5, Color::CYAN);

            let diff = point - transform.translation;

            let yaw = f32::atan2(diff.x, diff.z);
            let pitch = f32::atan2(diff.y, diff.z);

            look_state.target_yaw = yaw;
            look_state.target_pitch = pitch;
        }
    }
}

pub(crate) fn look_update(
    mut query_player: Query<
        (
            &PlayerLookOptions,
            &mut PlayerLookState
        ), With<PlayerEntity>
    >,
) {
    for (
        look_options,
        mut look_state
    ) in query_player.iter_mut() {
        look_state.yaw = look_state.target_yaw;
        look_state.pitch = look_state.target_pitch;
    }
}