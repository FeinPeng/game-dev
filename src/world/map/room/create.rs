use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision_group::{
        GROUP_BALL, GROUP_BRICK, GROUP_DOOR, GROUP_SENSOR_DEAD_ZONE, GROUP_TRANSPARANT_WALL,
        GROUP_WALL,
    },
    utils::anim_sprite::{AnimSprite, AnimSpriteTimer, AnimationIndices},
    world::map::{Index, SensorDoor, Wall},
    GameAssets, GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use super::loading::{LoadingData, RoomComponents};

#[derive(Component)]
pub struct Arena;

pub fn boss(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();

    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
    commands.entity(s_entity).insert_children(0, &[s_collider]);
}

pub fn combat(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);

    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}

pub fn post_boss(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);
    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}

pub fn pre_boss(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);
    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}

pub fn start(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        AnimSprite::default(),
        RoomComponents,
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);
    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}

pub fn treasure(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);
    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}

pub fn store(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
) {
    let arena_sheet = assets.arena01_texture.clone();
    let arena_layout = assets.arena_combat_01_layout.clone();
    loading_data.loading_assets.push(arena_sheet.clone().into());
    loading_data
        .loading_assets
        .push(arena_layout.clone().into());
    // 背景
    let animation_indices = AnimationIndices::new(0, 0);
    commands.spawn((
        Sprite::from_atlas_image(
            arena_sheet,
            TextureAtlas {
                layout: arena_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        animation_indices,
        AnimSpriteTimer::new(0.1),
        RoomComponents,
        AnimSprite::default(),
        Arena,
    ));

    // 边界墙
    let wall_thickness: f32 = 30.0;

    // 左墙
    let l_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0 - 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let l_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(l_entity).insert_children(0, &[l_collider]);

    // 右墙
    let r_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let r_collider = commands
        .spawn((
            Collider::cuboid(wall_thickness / 2.0, WINDOW_HEIGHT / 2.0),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(r_entity).insert_children(0, &[r_collider]);

    // 上墙
    let t_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, 0.0),
            Wall,
            Velocity::zero(),
            RoomComponents,
        ))
        .id();
    let t_collider = commands
        .spawn((
            Collider::cuboid(
                (WINDOW_WIDTH - (2.0 * wall_thickness) + 5.0) / 2.0,
                wall_thickness / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(1.0),
            Restitution::coefficient(1.0),
            CollisionGroups::new(GROUP_WALL, Group::all()),
            // (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC),
        ))
        .id();
    commands.entity(t_entity).insert_children(0, &[t_collider]);

    // 透明墙
    let d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness, 0.0),
        ))
        .id();
    let d_collider = commands
        .spawn((
            // 对砖块表现为墙
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            CollisionGroups::new(GROUP_TRANSPARANT_WALL, GROUP_BRICK),
        ))
        .id();
    commands.entity(d_entity).insert_children(0, &[d_collider]);

    // 球删除区域
    let s_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - wall_thickness * 2.0, 0.0),
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
        ))
        .id();
    commands.entity(s_entity).insert_children(0, &[s_collider]);
    // 生成门
    let l_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                -(WINDOW_WIDTH / 2.0) + 400.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let l_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(l_d_entity).add_child(l_d_collider);

    let r_d_entity = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 490.0,
                (WINDOW_HEIGHT / 2.0) - 15.0,
                2.0,
            ),
            RoomComponents,
        ))
        .id();
    let r_d_collider = commands
        .spawn((
            Collider::cuboid(65.0, 20.0),
            SensorDoor,
            Sensor,
            CollisionGroups::new(GROUP_DOOR, GROUP_BRICK),
            Index(0),
        ))
        .id();
    commands.entity(r_d_entity).add_child(r_d_collider);
}
