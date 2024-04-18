use bevy::prelude::Component;

pub enum PlayerToolType {
    None,
    Weapon(f32) // f32: Index
}


#[derive(Component)]
pub struct PlayerTool {
    tool_type: PlayerToolType
}

impl Default for PlayerTool {
    fn default() -> Self {
        return Self {
            tool_type: PlayerToolType::None
        }
    }
}