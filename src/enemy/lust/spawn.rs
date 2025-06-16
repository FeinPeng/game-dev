use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{lust::Lust, Enemy},
    GameAssets, GameState,
};

use super::{LustEnemy, LustState, SpawnLustEvent, LUST_HEIGHT, LUST_MASS, LUST_WIDTH};

pub struct LustSpawnPlugin;

impl Plugin for LustSpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnLustEvent>()
            .add_systems(Update, (spawn_lustes,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_lustes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_lustes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnLustEvent>,
) {
    for event in events.read() {
        spawn_lust(&mut commands, &assets, event.pos);
    }
}

fn despawn_lustes(
    mut commands: Commands,
    mut q_lustes: Query<(Entity, &Pressure, &mut LustState)>,
) {
    for (entity, lust_pressure, mut lust_state) in q_lustes.iter_mut() {
        if lust_pressure.current >= lust_pressure.max && *lust_state != LustState::Dying {
            *lust_state = LustState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_lust(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_Lust");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Lust,
            LustEnemy::default(),
            Sprite::from_image(assets.enemy_lust_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Lust,
            Collider::cuboid(LUST_WIDTH / 2.0, LUST_HEIGHT / 2.0),
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
