use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, IntoSystemConfigs};
use crate::plugins::camera::orbit3d::orbit3d_controller;
use crate::plugins::camera::offset::controller::{offset_controller, offset_controller_control};

pub mod orbit3d;
pub mod offset;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                orbit3d_controller,

                offset_controller_control,
                offset_controller.after(offset_controller_control)
            ));

        return;
    }
}