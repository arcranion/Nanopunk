use bevy::prelude::{Bundle, SpatialBundle};
use bevy_rapier3d::prelude::*;

use crate::plugins::player::physics::components::{PlayerPhysicsEntity, PlayerPhysicsOptions, PlayerPhysicsState};

#[derive(Bundle)]
pub struct PlayerPhysicsBundle {
    pub entity: PlayerPhysicsEntity,

    pub options: PlayerPhysicsOptions,
    pub state: PlayerPhysicsState,

    pub transform: SpatialBundle,
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub friction: Friction,
    pub restitution: Restitution,
    pub locked_axes: LockedAxes,
    pub ccd: Ccd,
    pub additional_mass_properties: AdditionalMassProperties,
    pub gravity_scale: GravityScale,
    pub sleeping: Sleeping,
    pub active_events: ActiveEvents,
}

impl Default for PlayerPhysicsBundle {
    fn default() -> Self {
        return Self {
            entity: PlayerPhysicsEntity,
            
            options: PlayerPhysicsOptions::default(),
            state: PlayerPhysicsState::default(),

            transform: SpatialBundle::default(),
            collider: Collider::default(),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ccd: Ccd::enabled(),
            additional_mass_properties: AdditionalMassProperties::Mass(1.0),
            gravity_scale: GravityScale(0.0),
            sleeping: Sleeping::disabled(),
            active_events: ActiveEvents::COLLISION_EVENTS
        }
    }
}