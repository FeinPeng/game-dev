mod collision;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 刚度系数
pub const STIFFNESS: f32 = 1000.0;
// 流体密度
pub const FLUID_DENSITY: f32 = 1.83;
// 马格努斯系数
pub const MAGNUS_COEFFICIENT: f32 = 0.3;
// 旋转阻尼系数
pub const DAMPING_COEFFICIENT: f32 = 0.8;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((collision::CollisionPlugin,));
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
#[require(OriginalVel)]
pub enum Ball {
    #[default]
    Tennis,
}

#[derive(Component)]
pub struct DamageCoefficient(pub f32);

#[derive(Component)]
pub struct TargetForce(pub Vec2);

impl Default for TargetForce {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component, PartialEq)]
#[require(Velocity)]
pub struct OriginalVel(pub Velocity);

impl Default for OriginalVel {
    fn default() -> Self {
        Self(Velocity::zero())
    }
}
