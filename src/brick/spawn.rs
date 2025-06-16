use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision_group::*,
    item::Item,
    utils::anim_sprite::{AnimSprite, AnimSpriteTimer, AnimationIndices},
    GameAssets, GameState,
};

use super::{
    item_collection::{self, ItemCollection},
    stats::*,
    Brick, Dimensions, Speed,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_brick);
    }
}

fn spawn_brick(mut commands: Commands, assets: Res<GameAssets>) {
    let animation_indices = AnimationIndices::new(0, 0);
    let brick_entity = commands
        .spawn((
            Speed(BRICK_SPEED),
            RigidBody::KinematicVelocityBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            GravityScale(0.0),
            Pressure::default(),
            Sprite::from_atlas_image(
                assets.brick_texture.clone(),
                TextureAtlas {
                    layout: assets.brick_layout.clone(),
                    index: animation_indices.first,
                },
            ),
            animation_indices,
            AnimSpriteTimer::new(0.1),
            AnimSprite {
                repeating: true,
                disabled: false,
            },
            Transform::from_xyz(0.0, -250.0, 1.0),
            Brick,
            Dimensions {
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
            },
            Friction::coefficient(3.0),
            Restitution::coefficient(3.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(BRICK_WIDTH / 2.0, BRICK_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            ColliderMassProperties::MassProperties(MassProperties {
                mass: BRICK_MASS,
                ..default()
            }),
            CollisionGroups::new(GROUP_BRICK, Group::all() ^ GROUP_SENSOR_DEAD_ZONE),
            // SolverGroups::new(GROUP_BRICK, Group::all() ^ GROUP_SENSOR_DEAD_ZONE),
            ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(3.0),
            Restitution::coefficient(3.0),
        ))
        .id();

    commands
        .entity(brick_entity)
        .insert_children(0, &[collider]);
}
