use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball::{Ball, OriginalVel},
    ui::AimAngle,
    GameAssets, GameState,
};

use super::{
    brick_ball::{spawn_ball, BRICK_BALL_SIZE},
    input::{PlayerInput, ToggleAimEvent},
    inventory::Inventory,
    stats::BRICK_HEIGHT,
    Brick,
};

const BALL_Y_OFFSET: f32 = BRICK_HEIGHT / 2.0 + BRICK_BALL_SIZE / 2.0 + 9.0;

#[derive(Component)]
pub struct BallInHand;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (prepare_ball, update_prepare_ball, shoot)
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

fn shoot(
    mut commands: Commands,
    mut player_input: ResMut<PlayerInput>,
    mut q_ball_in_hand: Query<
        (&mut Sleeping, &mut Velocity, &mut OriginalVel, Entity),
        With<BallInHand>,
    >,
    aim_angle: Res<AimAngle>,
) {
    if !player_input.shoot {
        return;
    }
    let aim_direction = Vec2::new(aim_angle.0.cos(), aim_angle.0.sin());
    player_input.toggle_aim = false;
    for (mut sleep, mut velocity, mut original_vel, entity) in q_ball_in_hand.iter_mut() {
        commands.entity(entity).remove::<BallInHand>();
        sleep.sleeping = false;
        *velocity = Velocity::linear(aim_direction * 500.0);
        original_vel.0 = *velocity;
    }
}

// TODO: 改为从背包里拿出一个球
fn prepare_ball(
    mut commands: Commands,
    assets: Res<GameAssets>,
    brick_transform: Single<&Transform, With<Brick>>,
    mut events: EventReader<ToggleAimEvent>,
    q_existion_ball_in_hand: Query<(Entity, &Ball), With<BallInHand>>,
    mut inventory: ResMut<Inventory>,
) {
    for event in events.read() {
        if event.0 {
            let in_hand_ball_position = Vec2::new(
                brick_transform.translation.x,
                brick_transform.translation.y + BALL_Y_OFFSET,
            );

            let ball = inventory.pop();
            if let Some(b) = ball {
                let ball_entity = spawn_ball(
                    &mut commands,
                    &assets,
                    b,
                    in_hand_ball_position,
                    Vec2::new(0.0, 0.0),
                    true,
                );
                commands.entity(ball_entity).insert(BallInHand);
            }
        } else {
            for (entity, ball) in q_existion_ball_in_hand.iter() {
                commands.entity(entity).despawn_recursive();
                inventory.push(*ball).expect("can't push ball");
            }
        }
    }
}

fn update_prepare_ball(
    brick_transform: Single<&Transform, (With<Brick>, Without<BallInHand>)>,
    mut q_existion_ball_in_hand: Query<&mut Transform, With<BallInHand>>,
) {
    let in_hand_ball_position = Vec2::new(
        brick_transform.translation.x,
        brick_transform.translation.y + BALL_Y_OFFSET,
    );

    for mut transform in q_existion_ball_in_hand.iter_mut() {
        *transform = Transform::from_translation(in_hand_ball_position.extend(1.0));
    }
}
