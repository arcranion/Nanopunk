use bevy::prelude::*;

use crate::plugins::player::{
    character::components::PlayerCharacterBundle,
    components::PlayerEntity,
    input::components::PlayerInputState,
    inventory::component::PlayerInventory,
    physics::bundle::PlayerPhysicsBundle,
    tool::PlayerTool
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: PlayerEntity,

    pub input_state: PlayerInputState,

    #[bundle()]
    pub physics: PlayerPhysicsBundle,
    #[bundle()]
    pub character: PlayerCharacterBundle,
    pub inventory: PlayerInventory,
    pub current_tool: PlayerTool
}

impl Default for PlayerBundle {
    fn default() -> Self {
        return Self {
            entity: PlayerEntity,

            input_state: PlayerInputState::default(),

            physics: PlayerPhysicsBundle::default(),

            character: PlayerCharacterBundle::default(),
            inventory: PlayerInventory::default(),
            current_tool: PlayerTool::default(),
        }
    }
}