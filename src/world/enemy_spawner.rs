use bevy::prelude::*;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(PreUpdate, (spawn_enemy).run_if(in_state(GameState::Gaming)));
    }
}

// fn spawn_enemy(mut event: EventWriter<SpawnSlothEvent>, keys: Res<ButtonInput<KeyCode>>) {
//     if keys.just_pressed(KeyCode::Space) {
//         event.send(SpawnSlothEvent { pos: Vec2::ZERO });
//         info!("enemy spawn");
//     }
// }
