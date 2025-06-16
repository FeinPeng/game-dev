use bevy::prelude::*;

use crate::GameState;

pub struct AnimSpritePlugin;

impl Plugin for AnimSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites.run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Component, Default)]
pub struct AnimSprite {
    pub repeating: bool,
    pub disabled: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimSpriteTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl Default for AnimSpriteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.075, TimerMode::Repeating))
    }
}

impl AnimSpriteTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimSprite,
        &mut AnimSpriteTimer,
        &mut Sprite,
    )>,
) {
    for (indices, mut anim_sprite, mut timer, mut sprite) in &mut query {
        if anim_sprite.disabled {
            timer.reset();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.first
            }
            anim_sprite.disabled = false;
            continue;
        }

        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    if anim_sprite.repeating {
                        indices.first
                    } else {
                        // anim_sprite.disabled = true;
                        indices.last
                    }
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
