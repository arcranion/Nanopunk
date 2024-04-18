use bevy::prelude::{Quat, Vec3};

pub struct WeaponValue {
    pub offset_position: Vec3,
    pub offset_rotation: Quat
}

impl Default for WeaponValue {
    fn default() -> Self {
        return Self {
            offset_position: Vec3::ZERO,
            offset_rotation: Quat::default()
        }
    }
}