use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{envy::Envy, Enemy},
    GameAssets, GameState,
};

use super::{EnvyEnemy, EnvyState, SpawnEnvyEvent, ENVY_HEIGHT, ENVY_MASS, ENVY_WIDTH};

pub struct EnvySpawnPlugin;

impl Plugin for EnvySpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnEnvyEvent>()
            .add_systems(Update, (spawn_envyes,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_envyes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_envyes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnEnvyEvent>,
) {
    for event in events.read() {
        spawn_envy(&mut commands, &assets, event.pos);
    }
}

fn despawn_envyes(
    mut commands: Commands,
    mut q_envyes: Query<(Entity, &Pressure, &mut EnvyState)>,
) {
    for (entity, envy_pressure, mut envy_state) in q_envyes.iter_mut() {
        if envy_pressure.current >= envy_pressure.max && *envy_state != EnvyState::Dying {
            *envy_state = EnvyState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_envy(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_ENVY");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Envy,
            EnvyEnemy::default(),
            Sprite::from_image(assets.enemy_envy_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Envy,
            Collider::cuboid(ENVY_WIDTH / 2.0, ENVY_HEIGHT / 2.0),
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
