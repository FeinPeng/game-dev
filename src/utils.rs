pub mod anim_sprite;
pub mod collections;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((anim_sprite::AnimSpritePlugin,));
    }
}
