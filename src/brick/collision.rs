use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ball::Ball;
use crate::GameState;

use super::Brick;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (brick_collision_handle).run_if(in_state(GameState::Gaming)),
        );
    }
}
fn brick_collision_handle(
    rapier_contex: ReadRapierContext,
    mut q_brick: Query<(&Children, &mut Transform), With<Brick>>,
    q_ball: Query<(), With<Ball>>,
    q_parent: Query<&Parent, With<Collider>>,
) {
    let (brick_children, mut brick_transform) = q_brick.get_single_mut().ok().unwrap();
    let child_entity = brick_children[0];

    for contact_pair in rapier_contex
        .single()
        .contact_pairs_with(child_entity)
        // 过滤掉所有Ball
        // TODO: 如果没有Wheel物品把Enemy也给过滤掉
        .filter(|contact_pair| {
            let collider_parent = q_parent.get(contact_pair.collider2()).ok().unwrap().get();
            if let Ok(_) = q_ball.get(collider_parent) {
                false
            } else {
                true
            }
        })
    {
        if child_entity == contact_pair.collider1() {
            for manifold in contact_pair.manifolds() {
                // println!("brick cotact!!!");
                let distance = if let Some(cv) = manifold.point(0) {
                    cv.dist().abs()
                } else {
                    0.0
                };
                brick_transform.translation += distance * manifold.local_n2().extend(1.0);
            }
        } else {
            for manifold in contact_pair.manifolds() {
                // println!("brick cotact!!!");
                let distance = if let Some(cv) = manifold.point(0) {
                    cv.dist().abs()
                } else {
                    0.0
                };
                brick_transform.translation += distance * manifold.local_n1().extend(1.0);
            }
        }
    }
}
