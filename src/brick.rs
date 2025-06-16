pub mod brick_ball;
pub mod input;
pub mod inventory;
pub mod item_collection;
pub mod spawn;
pub mod stats;

mod animation;
mod collision;
mod movement;
mod shoot;

pub use shoot::BallInHand;

use bevy::prelude::*;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Speed(pub f32);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            movement::MovementPlugin,
            spawn::SpawnPlugin,
            shoot::ShootPlugin,
            brick_ball::BrickBallPlugin,
            collision::CollisionPlugin,
            animation::AnimationPlugin,
            stats::StatsPlugin,
            inventory::InventoryPlugin,
            item_collection::ItemCollectionPlugin,
        ));
    }
}
