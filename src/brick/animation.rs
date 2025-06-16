use bevy::prelude::*;

use crate::{
    utils::anim_sprite::{AnimSprite, AnimationIndices},
    GameState,
};

use super::{stats::Pressure, Brick};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (change_pressure_animation).run_if(in_state(GameState::Gaming)),
        );
    }
}

fn change_pressure_animation(
    mut q_brick: Query<
        (&Pressure, &mut AnimationIndices, &mut AnimSprite),
        (With<Brick>, Changed<Pressure>),
    >,
) {
    for (pressure, mut idices, mut anim_sprite) in q_brick.iter_mut() {
        anim_sprite.disabled = true;
        *idices = match pressure.current {
            0.0..20.0 => AnimationIndices::new(0, 0),
            20.0..40.0 => AnimationIndices::new(1, 1),
            40.0..60.0 => AnimationIndices::new(2, 2),
            60.0..80.0 => AnimationIndices::new(3, 6),
            _ => AnimationIndices::new(7, 10),
        };
    }
}
