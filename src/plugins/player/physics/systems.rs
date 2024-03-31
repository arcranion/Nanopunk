use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::input::components::PlayerInputState;
use crate::plugins::player::input::systems::player_input;
use crate::plugins::player::physics::components::{PlayerPhysicsOptions, PlayerPhysicsState};



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
            &mut KinematicCharacterController,
            Option<&mut KinematicCharacterControllerOutput>,
            &mut PlayerPhysicsState,
            &mut PlayerInputState,
            &PlayerPhysicsOptions
        ), With<PlayerEntity>
    >,
    res_rapier_configuration: Res<RapierConfiguration>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (
        mut character_controller,
        mut character_controller_output,
        mut player_physics,
        mut player_input_state,
        player_options
    ) in query_player.iter_mut() {
        // Apply gravity
        player_physics.velocity += res_rapier_configuration.gravity * delta;

        // Apply friction
        player_physics.velocity /= 1.0 + player_options.friction_factor * delta;

        // Speed multiplied by delta time
        let speed = player_physics.speed * delta;

        // Calculate direction of movement
        let right = player_options.orientation_rotation * Vec3::X;
        let forward = player_options.orientation_rotation * Vec3::NEG_Z;
        let movement_vec3 = Vec3 {
            x: player_input_state.movement_normalized.x,
            y: 0.0,
            z: player_input_state.movement_normalized.y
        };
        let direction = (forward + right) * movement_vec3;
        
        if let Some(character_controller_output) = character_controller_output {
            // Apply movement
            player_physics.velocity += direction * if character_controller_output.grounded {
                speed
            } else {
                speed * player_options.midair_factor
            };

            // Jump if triggered
            if player_input_state.jumping && character_controller_output.grounded {
                player_physics.velocity.y += player_options.force_jump;
            }
        }

        println!("Velocity: {}", player_physics.velocity.to_string());

        // Apply the velocity to character controller
        character_controller.translation = Some(player_physics.velocity);
    }

    return;
}