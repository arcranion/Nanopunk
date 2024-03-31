use bevy::prelude::{Component, Entity};

use crate::core::item::InventoryItem;

#[derive(Component)]
pub struct PlayerInventory {
    items: Vec<InventoryItem>,
    weapons: Vec<Entity>,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        return Self {
            items: Vec::new(),
            weapons: Vec::new()
        }
    }
}