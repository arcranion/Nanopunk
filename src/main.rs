#![allow(warnings)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::Collider;

use crate::plugins::camera::CameraControllerPlugin;
use crate::plugins::camera::orbit3d::{Orbit3dCameraControllerBundle, Orbit3dCameraControllerEntity, Orbit3dCameraControllerOptions, Orbit3dCameraControllerState};
use crate::plugins::camera::orbit3d::Orbit3dCameraControllerTarget::OriginEntity;
use crate::plugins::input::InputPlugin;
use crate::plugins::physics::PhysicsPlugin;
use crate::plugins::player::bundle::PlayerBundle;
use crate::plugins::player::renderer::components::{PlayerRendererBundle, PlayerRendererOptions};
use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::physics::bundle::PlayerPhysicsBundle;
use crate::plugins::player::physics::components::PlayerPhysicsEntity;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::screen::ScreenPlugin;
use crate::plugins::state::StatePlugin;

pub mod plugins;
pub mod state;
pub mod core;
pub mod math;

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
    let player_character_model = asset_server.load("character.glb#Scene0");
    // let player_character_model = asset_server.load("renderer.glb#Scene0");

    // Spawn test plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(100.0, 1.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(prototype_texture.clone()),
            ..default()
        }),
        ..default()
    }, Collider::cuboid(50.0, 0.5, 50.0)));

    let player = commands.spawn((
        PlayerBundle {
            entity: PlayerEntity,
            input_state: Default::default(),
            physics: PlayerPhysicsBundle {
                entity: PlayerPhysicsEntity,

                transform: Transform::from_xyz(0.0, 50.0, 0.0).into(),
                options: Default::default(),
                state: Default::default(),
                collider: Collider::capsule_y(1.0, 0.5),
                velocity: Default::default(),
                ..default()
            },
            inventory: Default::default(),
            current_tool: Default::default(),
        }
    )).with_children(|parent| {
        parent.spawn(PlayerRendererBundle {
            model_scene: SceneBundle {
                scene: player_character_model,
                transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::splat(1.0)),
                ..default()
            },
            options: PlayerRendererOptions {
                head_name: "Head_M".to_string(),
            },
            ..default()
        });
    }).id();

    // Spawn top-view camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z),
            ..default()
        },
        Orbit3dCameraControllerBundle {
            entity: Orbit3dCameraControllerEntity,
            options: Orbit3dCameraControllerOptions {
                sensitivity: 1.0,
                distance_range: f32::MIN .. f32::MAX,
                distance_sensitivity: 10.0,
                interpolation_factor: 5.0,
            },
            target: OriginEntity(player),
            state: Orbit3dCameraControllerState {
                yaw: 0.0,
                pitch: 0.0,
                height: 0.0,
                yaw_target: 0.0,
                pitch_target: 0.0,
                height_target: 50.0,
            },
        },
        // OffsetCameraControllerBundle {
        //     options: OffsetCameraControllerOptions {
        //         zoom_factor: Vec3::NEG_Y,
        //         zoom_interpolation_factor: 20.0,
        //         translation_interpolation_factor: 5.0,
        //         offset: Vec3::new(0.0, 100.0, 0.0),
        //     },
        //     target: OffsetCameraControllerTarget::TargetEntity(player),
        //     ..default()
        // },
        // OffsetCameraControllerControl {
        //     zoom_action: Actions::DevZoomControl,
        //     zoom_sensitivity: 15.0,
        // }
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