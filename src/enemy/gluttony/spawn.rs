use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    brick::stats::{Pressure, BRICK_MASS},
    collision_group::{GROUP_ENEMY, GROUP_SENSOR_DEAD_ZONE},
    enemy::{gluttony::Gluttony, Enemy},
    GameAssets, GameState,
};

use super::{
    GluttonyEnemy, GluttonyState, SpawnGluttonyEvent, GLUTTONY_HEIGHT, GLUTTONY_MASS,
    GLUTTONY_WIDTH,
};

pub struct GluttonySpawnPlugin;

impl Plugin for GluttonySpawnPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnGluttonyEvent>()
            .add_systems(
                Update,
                (spawn_gluttonyes,).run_if(in_state(GameState::Gaming)),
            )
            .add_systems(
                PostUpdate,
                despawn_gluttonyes.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_gluttonyes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<SpawnGluttonyEvent>,
) {
    for event in events.read() {
        spawn_gluttony(&mut commands, &assets, event.pos);
    }
}

fn despawn_gluttonyes(
    mut commands: Commands,
    mut q_gluttonyes: Query<(Entity, &Pressure, &mut GluttonyState)>,
) {
    for (entity, gluttony_pressure, mut gluttony_state) in q_gluttonyes.iter_mut() {
        if gluttony_pressure.current >= gluttony_pressure.max
            && *gluttony_state != GluttonyState::Dying
        {
            *gluttony_state = GluttonyState::Dying;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_gluttony(commands: &mut Commands, assets: &Res<GameAssets>, spawn_pos: Vec2) {
    info!("in spawn_Gluttony");
    let entity = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Enemy::Gluttony,
            GluttonyEnemy::default(),
            Sprite::from_image(assets.enemy_gluttony_texture.clone()),
            Transform::from_translation(spawn_pos.extend(1.0)),
            GravityScale(0.0),
        ))
        .id();

    let collider = commands
        .spawn((
            Gluttony,
            Collider::cuboid(GLUTTONY_WIDTH / 2.0, GLUTTONY_HEIGHT / 2.0),
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
