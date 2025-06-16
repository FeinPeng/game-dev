use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{
        greed::{Greed, GREED_RADIUS},
        Enemy,
    },
    GameAssets, GameState,
};

use super::{GreedEnemy, GreedState, SpawnGreedEvent, GREED_MASS};

pub struct GreedSpawnPlugin;

impl Plugin for GreedSpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnGreedEvent>()
            .add_systems(Update, (spawn_greedes,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_greedes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_greedes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnGreedEvent>,
) {
    for event in events.read() {
        spawn_greed(&mut commands, &assets, event.pos);
    }
}

fn despawn_greedes(
    mut commands: Commands,
    mut q_greedes: Query<(Entity, &Pressure, &mut GreedState)>,
) {
    for (entity, greed_pressure, mut greed_state) in q_greedes.iter_mut() {
        if greed_pressure.current >= greed_pressure.max && *greed_state != GreedState::Dying {
            *greed_state = GreedState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_greed(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_Greed");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Greed,
            GreedEnemy::default(),
            Sprite::from_image(assets.enemy_greed_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Greed,
            Collider::ball(GREED_RADIUS),
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
