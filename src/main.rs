use std::ops::Range;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::transform::systems::propagate_transforms;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use crate::input::actions::Actions;
use crate::plugins::camera::CameraControllerPlugin;
use crate::plugins::camera::orbit3d::{Orbit3dCameraControllerBundle, Orbit3dCameraControllerEntity, Orbit3dCameraControllerOptions, Orbit3dCameraControllerState, Orbit3dCameraControllerTarget};
use crate::plugins::camera::offset::components::{OffsetCameraControllerBundle, OffsetCameraControllerControl, OffsetCameraControllerOptions, OffsetCameraControllerState, OffsetCameraControllerTarget};

use crate::plugins::input::InputPlugin;
use crate::plugins::physics::PhysicsPlugin;
use crate::plugins::player::components::bundle::PlayerBundle;
use crate::plugins::player::components::entity::{PlayerEntity, PlayerPhysics};
use crate::plugins::player::PlayerPlugin;

pub mod plugins;
mod input;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PhysicsPlugin,
            InputPlugin,
            CameraControllerPlugin
        ))
        .add_plugins(PlayerPlugin)
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
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.5, 2.0)),
            material: materials.add(StandardMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cylinder(1.0, 0.5),
        PlayerBundle {
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

fn update(q: Query<(&Transform, &PlayerPhysics), With<PlayerEntity>>) {
    let (tr, ph) = q.single();
}