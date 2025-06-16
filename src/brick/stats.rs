use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::GameState;

use super::{Brick, Dimensions};

// 砖块属性
pub const BRICK_WIDTH: f32 = 113.0;
pub const BRICK_HEIGHT: f32 = 38.0;
pub const BRICK_SPEED: f32 = 500.0;
pub const BRICK_MASS: f32 = 80.0;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrickStats>().add_systems(
            Update,
            update_dimensions.run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Resource)]
pub struct BrickStats {
    pub current_room: usize,
    pub num_exits: usize,
}

impl Default for BrickStats {
    fn default() -> Self {
        Self {
            current_room: 0_usize,
            num_exits: 0_usize,
        }
    }
}

#[derive(Component)]
pub struct Pressure {
    pub current: f32,
    pub max: f32,
}

impl Default for Pressure {
    fn default() -> Self {
        Self {
            current: 0.0,
            max: 100.0,
        }
    }
}

fn update_dimensions(
    mut q_brick: Query<
        (&mut Transform, &mut Collider, &Dimensions),
        (Changed<Dimensions>, With<Brick>),
    >,
) {
    for (mut transform, mut collider, dimensions) in q_brick.iter_mut() {
        let scale_x = dimensions.width / BRICK_WIDTH;
        let scale_y = dimensions.height / BRICK_HEIGHT;
        transform.scale = Vec3::new(scale_x, scale_y, 1.0);

        *collider = Collider::cuboid(dimensions.width / 2.0, dimensions.height / 2.0);
    }
}
