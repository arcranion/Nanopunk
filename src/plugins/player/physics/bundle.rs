use bevy::prelude::Bundle;
use bevy_rapier3d::control::KinematicCharacterController;

use crate::plugins::player::physics::components::{PlayerPhysicsEntity, PlayerPhysicsOptions, PlayerPhysicsState};

#[derive(Bundle)]
pub struct PlayerPhysicsBundle {
    pub entity: PlayerPhysicsEntity,
    pub options: PlayerPhysicsOptions,
    pub physics: PlayerPhysicsState,

    pub character_controller: KinematicCharacterController
}

impl Default for PlayerPhysicsBundle {
    fn default() -> Self {
        return Self {
            entity: PlayerPhysicsEntity,
            
            options: PlayerPhysicsOptions::default(),
            physics: PlayerPhysicsState::default(),

            character_controller: KinematicCharacterController::default()
        }
    }
}