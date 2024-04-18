use bevy::prelude::*;

use crate::plugins::player::{
    components::PlayerEntity,
    input::components::PlayerInputState,
    inventory::component::PlayerInventory,
    physics::bundle::PlayerPhysicsBundle,
    tool::PlayerTool
};
use crate::plugins::player::look::components::PlayerLookBundle;
use crate::plugins::player::renderer::components::PlayerRendererBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: PlayerEntity,

    pub input_state: PlayerInputState,

    #[bundle()]
    pub look: PlayerLookBundle,
    #[bundle()]
    pub physics: PlayerPhysicsBundle,
    pub inventory: PlayerInventory,
    pub current_tool: PlayerTool
}

impl Default for PlayerBundle {
    fn default() -> Self {
        return Self {
            entity: PlayerEntity,

            input_state: PlayerInputState::default(),

            look: PlayerLookBundle::default(),
            physics: PlayerPhysicsBundle::default(),

            inventory: PlayerInventory,
            current_tool: PlayerTool::default(),
        }
    }
}