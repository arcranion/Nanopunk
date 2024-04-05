use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component, Entity};

use crate::plugins::input::actions::Actions;

// Bundle
#[derive(Bundle)]
pub struct OffsetCameraControllerBundle {
    pub entity: OffsetCameraControllerEntity,
    pub options: OffsetCameraControllerOptions,
    pub target: OffsetCameraControllerTarget,
    pub state: OffsetCameraControllerState
}

impl Default for OffsetCameraControllerBundle {
    fn default() -> Self {
        return Self {
            entity: OffsetCameraControllerEntity,
            options: OffsetCameraControllerOptions::default(),
            target: OffsetCameraControllerTarget::default(),
            state: OffsetCameraControllerState::default()
        }
    }
}

// Marker
#[derive(Component)]
pub struct OffsetCameraControllerEntity;

// Options
#[derive(Component)]
pub struct OffsetCameraControllerOptions {
    pub zoom_factor: Vec3,

    pub zoom_interpolation_factor: f32,
    pub translation_interpolation_factor: f32,

    pub offset: Vec3,
}

impl Default for OffsetCameraControllerOptions {
    fn default() -> Self {
        return Self {
            zoom_factor: Vec3::NEG_Y,

            zoom_interpolation_factor: 1.0,
            translation_interpolation_factor: 1.0,

            offset: Vec3::ZERO,
        }
    }
}

// Target Enum
#[derive(Component)]
pub enum OffsetCameraControllerTarget {
    TargetEntity(Entity),
    TargetVector(Vec3)
}

impl OffsetCameraControllerTarget {
    fn zero() -> OffsetCameraControllerTarget {
        return OffsetCameraControllerTarget::TargetVector(Vec3::ZERO);
    }
}

impl Default for OffsetCameraControllerTarget {
    fn default() -> Self {
        return Self::zero();
    }
}

// State
#[derive(Component)]
pub struct OffsetCameraControllerState {
    pub zoom_target: f32,

    pub(super) zoom_internal: f32,
}

impl Default for OffsetCameraControllerState {
    fn default() -> Self {
        return Self {
            zoom_target: 0.0,
            zoom_internal: 0.0
        }
    }
}

// Control
#[derive(Component)]
pub struct OffsetCameraControllerControl {
    pub zoom_action: Actions,
    pub zoom_sensitivity: f32
}