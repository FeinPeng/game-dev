mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const ENVY_WIDTH: f32 = 70.0;
pub const ENVY_HEIGHT: f32 = 50.0;
pub const ENVY_MASS: f32 = 150.0;

pub struct EnvyPlugin;

impl Plugin for EnvyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnEnvyEvent>()
            .add_plugins((spawn::EnvySpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum EnvyState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Envy;

#[derive(Bundle)]
pub struct EnvyEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: EnvyState,
}

impl Default for EnvyEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: EnvyState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnEnvyEvent {
    pub pos: Vec2,
}
