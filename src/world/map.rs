pub mod pipelines_readdy;
pub mod room;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use room::control::ChooseState;
use room::create::Arena;
use room::loading::{LoadingData, LoadingState, RoomComponents};
use room::select::SelectedRooms;

use crate::audio::PlaySound;
use crate::audio::Volume;
use crate::ball::Ball;
use crate::brick::{stats::BrickStats, Brick};
use crate::collision_group::*;
use crate::enemy::envy::SpawnEnvyEvent;
use crate::enemy::gluttony::SpawnGluttonyEvent;
use crate::enemy::greed::SpawnGreedEvent;
use crate::enemy::sloth::SpawnSlothEvent;
use crate::events::{Damage, DamageEvent};
use crate::item::sapwn::SpawnItemEvent;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer, AnimationIndices};
use crate::{GameAssets, GameState};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_map,))
            .add_systems(
                Update,
                (handle_sensor_wall).run_if(in_state(GameState::Gaming)),
            )
            .add_systems(
                Update,
                (handle_sensor_door)
                    .run_if(in_state(ChooseState::Choosing))
                    .run_if(in_state(GameState::Gaming)),
            )
            .add_systems(OnEnter(ChooseState::Choosing), active_sensor_door)
            .add_plugins((room::RoomPlugin,));
    }
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct SensorWall;

#[derive(Component)]
pub struct SensorDoor;

#[derive(Component)]
pub struct Index(pub usize);

fn spawn_map(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut loading_data: ResMut<LoadingData>,
    mut event: EventWriter<SpawnGluttonyEvent>,
    mut brick_state: ResMut<BrickStats>,
    mut spawn_item_event_writer: EventWriter<SpawnItemEvent>,
) {
    brick_state.current_room = 0;
    brick_state.num_exits = 2;
    event.send(SpawnGluttonyEvent {
        pos: Vec2::new(0.0, 0.0),
    });
    spawn_item_event_writer.send(SpawnItemEvent {
        pos: Vec2::new(60.0, 60.0),
    });

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
        AnimSprite {
            repeating: false,
            disabled: false,
        },
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
            Friction::coefficient(0.5),
            Restitution::coefficient(1.0),
            ColliderMassProperties::MassProperties(MassProperties {
                mass: f32::MAX,
                ..default()
            }),
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
            Friction::coefficient(0.5),
            Restitution::coefficient(1.0),
            ColliderMassProperties::MassProperties(MassProperties {
                mass: f32::MAX,
                ..default()
            }),
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
            Friction::coefficient(0.5),
            Restitution::coefficient(1.0),
            ColliderMassProperties::MassProperties(MassProperties {
                mass: f32::MAX,
                ..default()
            }),
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
            RoomComponents,
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
            RoomComponents,
        ))
        .id();
    let s_collider = commands
        .spawn((
            // 对球表现为触发器
            Collider::cuboid(WINDOW_WIDTH / 2.0, wall_thickness),
            Sensor,
            CollisionGroups::new(GROUP_SENSOR_DEAD_ZONE, GROUP_BALL),
            SensorWall,
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

fn handle_sensor_wall(
    // mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut enents: EventWriter<DamageEvent>,
    brick_entity: Single<Entity, With<Brick>>,
    sensor_query: Query<(), (With<Sensor>, With<SensorWall>)>,
    q_ball: Query<(), With<Ball>>,
    q_damage: Query<&Damage>,
    q_parent: Query<&Parent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = event {
            if sensor_query.contains(*entity_b) {
                let parent_entity = q_parent.get(*entity_a).ok().unwrap().get();
                enents.send(DamageEvent {
                    offender: *entity_a,
                    victim: *brick_entity,
                    damage: *q_damage.get(parent_entity).ok().unwrap(),
                });
                // commands.entity(parent_entity).despawn_recursive();
                println!("despawn ball");
            }
            if sensor_query.contains(*entity_a) {
                let parent_entity = q_parent.get(*entity_b).ok().unwrap().get();
                enents.send(DamageEvent {
                    offender: *entity_b,
                    victim: *brick_entity,
                    damage: *q_damage.get(parent_entity).ok().unwrap(),
                });
                // commands.entity(parent_entity).despawn_recursive();
                println!("despawn ball");
            }
        }
    }
}

fn active_sensor_door(mut commands: Commands, q_door: Query<Entity, With<SensorDoor>>) {
    for entity in q_door.iter() {
        commands
            .entity(entity)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn handle_sensor_door(
    mut collision_events: EventReader<CollisionEvent>,
    q_sensor: Query<(), (With<Sensor>, With<SensorDoor>)>,
    q_brick: Query<(), With<Brick>>,
    q_index: Query<&Index>,
    q_parent: Query<&Parent>,
    mut next_choose_state: ResMut<NextState<ChooseState>>,
    mut next_loading_state: ResMut<NextState<LoadingState>>,
    mut selected_rooms: ResMut<SelectedRooms>,
    mut sound_events: EventWriter<PlaySound>,
    game_assets: Res<GameAssets>,
    volume: Res<Volume>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, CollisionEventFlags::SENSOR) = event {
            if q_sensor.contains(*entity_a) {
                // a 为 door b other
                if let Ok(parent) = q_parent.get(*entity_b) {
                    let parent_entity = parent.get();
                    if q_brick.contains(parent_entity) {}
                    let index = q_index.get(*entity_a).unwrap();
                    selected_rooms.index = index.0;
                    next_choose_state.set(ChooseState::Ready);
                    next_loading_state.set(LoadingState::FadeOut);
                    sound_events.send(PlaySound {
                        clip: game_assets.close_door_sound.clone(),
                        volume: volume.0 as f64,
                        ..default()
                    });
                }
            }
            if q_sensor.contains(*entity_b) {
                // b 为 door a other
                if let Ok(parent) = q_parent.get(*entity_a) {
                    let parent_entity = parent.get();
                    if q_brick.contains(parent_entity) {
                        let index = q_index.get(*entity_b).unwrap();
                        selected_rooms.index = index.0;
                        next_choose_state.set(ChooseState::Ready);
                        next_loading_state.set(LoadingState::FadeOut);
                        sound_events.send(PlaySound {
                            clip: game_assets.close_door_sound.clone(),
                            volume: volume.0 as f64,
                            ..default()
                        });
                    }
                }
            }
        }
    }
}
