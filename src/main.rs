use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::input::actions::Actions;
use crate::plugins::camera::CameraControllerPlugin;
use crate::plugins::camera::offset::components::{OffsetCameraControllerBundle, OffsetCameraControllerControl, OffsetCameraControllerOptions, OffsetCameraControllerTarget};
use crate::plugins::input::InputPlugin;
use crate::plugins::physics::PhysicsPlugin;
use crate::plugins::player::bundle::PlayerBundle;
use crate::plugins::player::character::components::PlayerCharacterBundle;
use crate::plugins::player::physics::bundle::PlayerPhysicsBundle;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::screen::ScreenPlugin;
use crate::plugins::state::StatePlugin;

pub mod plugins;
pub mod input;
pub mod state;
mod core;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            // EguiPlugin,
            WorldInspectorPlugin::new()
        ))
        .add_plugins((
            PhysicsPlugin,
            InputPlugin,
            CameraControllerPlugin,
            ScreenPlugin,
            StatePlugin,
            PlayerPlugin
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();

    return;
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let prototype_texture = asset_server.load("prototype_texture1.png");
    let player_character_model = asset_server.load("fox.glb#Scene0");

    // Spawn test plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(100.0, 1.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(prototype_texture.clone()),
            ..default()
        }),
        ..default()
    }, Collider::cuboid(50.0, 0.5, 50.0)));

    // Spawn player with mock model(capsule)
    let player = commands.spawn((
        // PbrBundle {
        //     mesh: meshes.add(Cylinder::new(0.5, 2.0)),
        //     material: materials.add(StandardMaterial::from(Color::WHITE)),
        //     transform: Transform::from_xyz(0.0, 5.0, 0.0),
        //     ..default()
        // },
        RigidBody::KinematicPositionBased,
        Collider::cylinder(1.0, 0.5),
        PlayerBundle {
            character: PlayerCharacterBundle {
                scene: SceneBundle {
                    scene: player_character_model,
                    transform: Transform::from_xyz(0.0, 50.0, 0.0).with_scale(Vec3::splat(0.1)),
                    ..default()
                }
            },
            ..default()
        }
    )).id();

    // Spawn top-view camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z),
                // .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0,  0.0, 0.0)),
            ..default()
        },
        OffsetCameraControllerBundle {
            options: OffsetCameraControllerOptions {
                zoom_factor: Vec3::NEG_Y,
                zoom_interpolation_factor: 20.0,
                translation_interpolation_factor: 5.0,
                offset: Vec3::new(0.0, 100.0, 0.0),
            },
            target: OffsetCameraControllerTarget::TargetEntity(player),
            ..default()
        },
        OffsetCameraControllerControl {
            zoom_action: Actions::DevZoomControl,
            zoom_sensitivity: 15.0,
        }
    ));

    // Spawn a directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    return;
}

fn update() {

}