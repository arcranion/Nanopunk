use bevy::math::Quat;
use bevy::prelude::Vec3;
use bevy::utils::default;
use crate::values::weapons::structures::WeaponValue;

const WEAPON_REVOLVER: WeaponValue = WeaponValue {
    offset_position: Vec3::ZERO,
    offset_rotation: Quat::IDENTITY
};