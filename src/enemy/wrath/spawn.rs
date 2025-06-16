use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{wrath::Wrath, Enemy},
    GameAssets, GameState,
};

use super::{SpawnWrathEvent, WrathEnemy, WrathState, WRATH_HEIGHT, WRATH_MASS, WRATH_WIDTH};

pub struct WrathSpawnPlugin;

impl Plugin for WrathSpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnWrathEvent>()
            .add_systems(Update, (spawn_wrathes,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_wrathes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_wrathes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnWrathEvent>,
) {
    for event in events.read() {
        spawn_wrath(&mut commands, &assets, event.pos);
    }
}

fn despawn_wrathes(
    mut commands: Commands,
    mut q_wrathes: Query<(Entity, &Pressure, &mut WrathState)>,
) {
    for (entity, wrath_pressure, mut wrath_state) in q_wrathes.iter_mut() {
        if wrath_pressure.current >= wrath_pressure.max && *wrath_state != WrathState::Dying {
            *wrath_state = WrathState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_wrath(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_Wrath");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Wrath,
            WrathEnemy::default(),
            Sprite::from_image(assets.enemy_wrath_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Wrath,
            Collider::cuboid(WRATH_WIDTH / 2.0, WRATH_HEIGHT / 2.0),
            ColliderMassProperties::MassProperties(MassProperties {
                mass: BRICK_MASS,
                ..default()
            }),
            CollisionGroups::new(GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE ^ Group::all()),
            ActiveEvents::COLLISION_EVENTS,
            Friction {
                coefficient: 0.5,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution::coefficient(1.0),
        ))
        .id();

    commands.entity(entity).insert_children(0, &[collider]);
}
