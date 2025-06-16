use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    audio::PlaySound,
    audio::Volume,
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{sloth::Sloth, Enemy},
    GameAssets, GameState,
};

use super::{SlothEnemy, SlothState, SpawnSlothEvent, SLOTH_HEIGHT, SLOTH_WIDTH};

pub struct SlothSpawnPlugin;

impl Plugin for SlothSpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnSlothEvent>()
            .add_systems(Update, (spawn_slothes,).run_if(in_state(GameState::Gaming)))
            .add_systems(
                PostUpdate,
                despawn_slothes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_slothes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnSlothEvent>,
) {
    for event in events.read() {
        spawn_sloth(&mut commands, &assets, event.pos);
    }
}

fn despawn_slothes(
    mut commands: Commands,
    mut q_slothes: Query<(Entity, &Pressure, &mut SlothState)>,
    mut sound_events: EventWriter<PlaySound>,
    game_assets: Res<GameAssets>,
    volume: Res<Volume>,
) {
    for (entity, sloth_pressure, mut sloth_state) in q_slothes.iter_mut() {
        if sloth_pressure.current >= sloth_pressure.max && *sloth_state != SlothState::Dying {
            *sloth_state = SlothState::Dying;
            sound_events.send(PlaySound {
                clip: game_assets.brick_cracked_sound.clone(),
                volume: volume.0 as f64,
                ..default()
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_sloth(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_sloth");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Sloth,
            SlothEnemy::default(),
            Sprite::from_image(assets.enemy_sloth_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Sloth,
            Collider::cuboid(SLOTH_WIDTH / 2.0, SLOTH_HEIGHT / 2.0),
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
