use std::fmt::Pointer;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::input::components::PlayerInputState;
use crate::plugins::player::physics::components::{PlayerPhysicsOptions, PlayerPhysicsState};

pub(super) fn player_update_speed(
    mut query_player: Query<(
        &PlayerInputState,
        &PlayerPhysicsOptions,
        &mut PlayerPhysicsState
    ), With<PlayerEntity>>,
    time: Res<Time>
) {
    for (
        input_state,
        physics_options,
        mut physics_state
    ) in query_player.iter_mut() {
        physics_state.target_speed = if input_state.crouch {
            physics_options.crouched_speed
        } else if input_state.sprint {
            physics_options.run_speed
        } else {
            physics_options.walk_speed
        }
    }
}

pub(super) fn player_look(
    mut query_player: Query<
        (
            &PlayerInputState,
            &PlayerPhysicsOptions,
            &mut Transform,
        ), With<PlayerEntity>
    >,
    query_camera: Query<
        (
            &Camera,
            &GlobalTransform
        )
    >,
    mut gizmos: Gizmos
) {
    let (camera, camera_global_transform) = query_camera.single();

    for (input_state, physics_options, mut transform) in query_player.iter_mut() {
        let point = camera
            .viewport_to_world(camera_global_transform, input_state.pointer)
            .map(|ray| {
                let distance = if let Some(distance) = ray.intersect_plane(transform.translation, Plane3d::default()) {
                    distance
                } else {
                    physics_options.pointer_distance_max
                };

                return ray.get_point(distance);
            });

        if let Some(point) = point {
            gizmos.sphere(point, Quat::IDENTITY, 5.0, Color::CYAN);

            let diff = point - transform.translation;
            let diff = diff.normalize();

            // let orient = diff;
            // let cross = Vec3::Y;
            let orient = Vec3::Y;
            let cross = diff;

            let axis = orient.cross(cross).normalize();

            let angle = orient.dot(cross).acos();

            let rotation = Quat::from_axis_angle(axis, angle);

            transform.rotation = rotation;
        }
    }
}

pub(super) fn player_movement(
    mut query_player: Query<
        (
            Entity,
            &mut Collider,
            &mut Transform,
            &mut Velocity,
            &mut PlayerPhysicsState,
            &mut PlayerInputState,
            &PlayerPhysicsOptions
        ), With<PlayerEntity>
    >,
    res_rapier_context: Res<RapierContext>,
    res_rapier_configuration: Res<RapierConfiguration>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();

    for (
        entity,
        mut collider,
        mut transform,
        mut velocity,
        mut physics_state,
        input_state,
        physics_options
    ) in query_player.iter_mut() {
        if let Some(capsule) = collider.as_capsule() {
        // if true {
            let capsule = capsule.raw;

            let capsule_collider = Collider::capsule(
                capsule.segment.a.into(),
                capsule.segment.b.into(),
                capsule.radius * 0.9
            );

            let cast_collider = capsule_collider;

            let filter = QueryFilter::default().exclude_rigid_body(entity);
            let ground_cast = res_rapier_context.cast_shape(
                transform.translation,
                transform.rotation,
                -Vec3::Y,
                &cast_collider,
                0.125,
                true,
                filter
            );

            let direction_factor = physics_options.direction_factor;

            let mut direction = physics_options.orientation_rotation * input_state.movement * direction_factor;
            let mut length = direction.length();

            if length > f32::EPSILON {
                direction /= length;
            }

            let max_speed = physics_state.target_speed;

            length = f32::min(length, max_speed);

            if let Some((toi, toi_details)) = unwrap_toi_details(ground_cast) {
                let has_traction = Vec3::dot(toi_details.normal1, Vec3::Y)
                    > physics_options.traction_normal_cutoff;

                let lateral_speed = velocity.linvel.xz().length();
                if lateral_speed > physics_options.friction_speed_cutoff {
                    let control = f32::max(lateral_speed, physics_options.stop_speed);
                    let drop = control * physics_options.friction * delta;
                    let new_speed =
                        f32::max((lateral_speed - drop) / lateral_speed, 0.0);
                    velocity.linvel.x *= new_speed;
                    velocity.linvel.z *= new_speed;
                } else {
                    velocity.linvel = Vec3::ZERO;
                }

                if physics_state.ground_time == 1 {
                    velocity.linvel.y = -toi.toi;
                }

                let mut acceleration = calculate_acceleration(
                    direction,
                    length,
                    physics_options.acceleration,
                    velocity.linvel,
                    delta,
                );

                velocity.linvel += acceleration;

                if !has_traction {
                    acceleration += res_rapier_configuration.gravity * delta;
                } else {
                    let linvel = velocity.linvel;
                    velocity.linvel -=
                        Vec3::dot(linvel, toi_details.normal1) * toi_details.normal1;

                    if input_state.jump {
                        velocity.linvel.y = physics_options.jump_force;
                    }
                }

                // Increment ground tick but cap at max value
                physics_state.ground_time = physics_state.ground_time.saturating_add(1);
            } else {
                physics_state.ground_time = 0;
                length = f32::min(length, physics_options.air_speed_cap);

                let mut add = calculate_acceleration(
                    direction,
                    length,
                    physics_options.air_acceleration,
                    velocity.linvel,
                    delta,
                );
                add.y = res_rapier_configuration.gravity.y * delta;
                velocity.linvel += add;

                let air_speed = velocity.linvel.xz().length();
                if air_speed > physics_options.max_air_speed {
                    let ratio = physics_options.max_air_speed / air_speed;
                    velocity.linvel.x *= ratio;
                    velocity.linvel.z *= ratio;
                }
            }
        }
    }
    

    return;
}

fn unwrap_toi_details(ground_cast: Option<(Entity, Toi)>) -> Option<(Toi, ToiDetails)> {
    if let Some((_, toi)) = ground_cast {
        if let Some(details) = toi.details {
            return Some((toi, details));
        }
    }

    return None;
}

fn calculate_acceleration(
    wish_direction: Vec3,
    wish_speed: f32,
    acceleration: f32,
    velocity: Vec3,
    dt: f32,
) -> Vec3 {
    let velocity_projection = Vec3::dot(velocity, wish_direction);
    let add_speed = wish_speed - velocity_projection;
    if add_speed <= 0.0 {
        return Vec3::ZERO;
    }

    let acceleration_speed = f32::min(acceleration * wish_speed * dt, add_speed);

    return wish_direction * acceleration_speed;
}