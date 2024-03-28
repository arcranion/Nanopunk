use bevy::prelude::Bundle;
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use crate::plugins::player::components::entity::{PlayerEntity, PlayerInputState, PlayerOptions, PlayerPhysics};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: PlayerEntity,
    pub options: PlayerOptions,
    pub input_state: PlayerInputState,
    pub physics: PlayerPhysics,

    pub character_controller: KinematicCharacterController
}

impl Default for PlayerBundle {
    fn default() -> Self {
        return Self {
            entity: PlayerEntity,
            options: PlayerOptions::default(),
            input_state: PlayerInputState::default(),
            physics: PlayerPhysics::default(),

            character_controller: KinematicCharacterController::default()
        }
    }
}