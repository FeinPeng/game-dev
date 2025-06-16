use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::GameState;

use super::{input::PlayerInput, stats::*, Brick, Speed};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (brick_movement,).run_if(in_state(GameState::Gaming)),
        );
    }
}

fn brick_movement(
    mut brick_velocity: Single<&mut Velocity, With<Brick>>,
    brick_speed: Single<&Speed, With<Brick>>,
    player_input: Res<PlayerInput>,
) {
    // // 计算当前速度和目标方向的夹角，如果为-1（夹角为180°）则直接改变速度而不必插值，这样操作会更流畅
    // let cos = brick_velocity.linvel.dot(player_input.move_direction)
    //     / (brick_velocity.linvel.length() * player_input.move_direction.length());

    // // 浮点数比较要注意精度问题
    // if (cos + 1.0).abs() < f32::EPSILON {
    //     brick_velocity.linvel = player_input.move_direction * BRICK_SPEED;
    // } else {
    //     brick_velocity.linvel = brick_velocity
    //         .linvel
    //         .lerp(player_input.move_direction * BRICK_SPEED, 0.4);
    // }
    brick_velocity.linvel = brick_velocity
        .linvel
        .lerp(player_input.move_direction * brick_speed.0, 0.5);
}
