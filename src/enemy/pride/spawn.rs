use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{pride::Pride, Enemy},
    GameAssets, GameState,
};

use super::{PrideEnemy, PrideState, SpawnPrideEvent, PRIDE_HEIGHT, PRIDE_MASS, PRIDE_WIDTH};

pub struct PrideSpawnPlugin;

impl Plugin for PrideSpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnPrideEvent>()
            .add_systems(Update, (spawn_pridees,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_pridees.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_pridees(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnPrideEvent>,
) {
    for event in events.read() {
        spawn_pride(&mut commands, &assets, event.pos);
    }
}

fn despawn_pridees(
    mut commands: Commands,
    mut q_pridees: Query<(Entity, &Pressure, &mut PrideState)>,
) {
    for (entity, pride_pressure, mut pride_state) in q_pridees.iter_mut() {
        if pride_pressure.current >= pride_pressure.max && *pride_state != PrideState::Dying {
            *pride_state = PrideState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_pride(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_Pride");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Pride,
            PrideEnemy::default(),
            Sprite::from_image(assets.enemy_pride_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Pride,
            Collider::cuboid(PRIDE_WIDTH / 2.0, PRIDE_HEIGHT / 2.0),
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
