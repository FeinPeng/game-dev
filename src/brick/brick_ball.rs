use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball::{Ball, DamageCoefficient, TargetForce},
    collision_group::{GROUP_BALL, GROUP_TRANSPARANT_WALL},
    events::Damage,
    world::map::room::loading::RoomComponents,
    GameAssets,
};

// 球属性
pub const BRICK_BALL_SIZE: f32 = 40.0;
pub const BRICK_BALL_MASS: f32 = 1.0;
pub const BRICK_BALL_ANGULAR_INERTIAL: f32 = 0.3;

pub struct BrickBallPlugin;

impl Plugin for BrickBallPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (brick_ball_collision_handle).run_if(in_state(GameState::Gaming)),
        // );
    }
}

pub fn spawn_ball(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    ball_type: Ball,
    position: Vec2,
    velosity: Vec2,
    is_sleeping: bool,
) -> Entity {
    let entity = commands
        .spawn((
            Sprite::from_image(assets.tennis_texture.clone()),
            RigidBody::KinematicVelocityBased,
            Ccd::enabled(),
            GravityScale(0.0),
            Velocity::linear(velosity),
            Sleeping {
                sleeping: is_sleeping,
                ..default()
            },
            Damage { value: 10.0 },
            Transform::from_translation(position.extend(1.0)),
            ball_type,
            RoomComponents,
            DamageCoefficient(1.0),
            TargetForce::default(),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(BRICK_BALL_SIZE / 2.0),
            ColliderMassProperties::MassProperties(MassProperties {
                mass: BRICK_BALL_MASS,
                principal_inertia: BRICK_BALL_ANGULAR_INERTIAL,
                ..default()
            }),
            ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_BALL, Group::all() ^ GROUP_TRANSPARANT_WALL),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();
    commands.entity(entity).insert_children(0, &[collider]).id()
}
