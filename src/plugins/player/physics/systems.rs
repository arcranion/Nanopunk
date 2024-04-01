use std::fmt::Pointer;
use std::ops::Deref;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::input::components::PlayerInputState;
use crate::plugins::player::input::systems::player_input;
use crate::plugins::player::physics::components::{PlayerCollider, PlayerPhysicsOptions, PlayerPhysicsState, PlayerTransform, PlayerVelocity};



pub(super) fn player_update_speed(
    mut query_player: Query<(
        &PlayerInputState,
        &PlayerPhysicsOptions,
        &mut PlayerPhysicsState
    ), With<PlayerEntity>>,
    time: Res<Time>
) {
    for (state, options, mut physics) in query_player.iter_mut() {
        let target_speed = if state.crouching { // Crouching
            options.speed_crouching
        } else if state.sprinting { // Sprinting
            options.speed_sprinting
        } else { // Walking
            options.speed_walking
        };

        physics.speed = physics.speed.lerp(target_speed, options.speed_interpolation_factor * time.delta_seconds());
    }
}

pub(super) fn player_look(
    
) {

}

pub(super) fn player_movement(
    mut query_player: Query<
        (
            Entity,
            &Collider,
            &Transform,
            &Velocity,
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
        collider,
        transform,
        velocity,
        physics_state,
        input_state,
        physics_options
    ) in query_player.iter_mut() {
        if let Some(capsule) = collider.as_capsule() {
            let capsule = capsule.raw;

            let capsule_collider = Collider::capsule(
                capsule.segment.a.into(),
                capsule.segment.b.into(),
                capsule.radius * 0.9
            );

            let filter = QueryFilter::default().exclude_rigid_body(entity);
            let ground_cast = res_rapier_context.cast_shape(
                transform.translation,
                transform.rotation,
                -Vec3::Y,
                &capsule_collider,
                0.125,
                true,
                filter
            );

            let speeds = Vec3::new(physics_options.side_speed, 0.0, physics_options.forward_speed);
            let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input_state.yaw);
            move_to_world.z_axis *= -1.0;

            let mut wish_direction = move_to_world * (input_state.movement * speeds);
            let mut wish_speed = wish_direction.length();

            if wish_speed > f32::EPSILON {
                wish_direction /= wish_speed;
            }

            let max_speed = if input_state.crouch {
                physics_options.crouched_speed
            } else if input_state.sprint {
                physics_options.run_speed
            } else {
                physics_options.walk_speed
            };

            wish_speed = f32::min(wish_speed, max_speed);

            if let Some((toi, toi_details)) = unwrap_toi_details(ground_cast) {
                let has_reaction = Vec3::dot(toi_details.normal1, Vec3::Y) > physics_options.traction_normal_cutoff;

                
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