use bevy::prelude::Entity;

pub struct InventoryItem;
pub struct InventoryWeapon;

pub struct InventoryData {
    assigned_entity: Entity,
    meta: InventoryMetaData
}

pub enum InventoryMetaData {
    Player {
        assigned: Entity,
        contents: Vec<Option<InventoryItem>>,
        weapons: Vec<InventoryWeapon>,
    }
}

pub struct InventoryDataResources {
    data: Vec<InventoryData>
}