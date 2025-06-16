mod camera;
mod enemy_spawner;
pub mod map;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
            enemy_spawner::EnemySpawnerPlugin,
        ));
    }
}
