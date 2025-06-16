mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const WRATH_WIDTH: f32 = 130.0;
pub const WRATH_HEIGHT: f32 = 60.0;
pub const WRATH_MASS: f32 = 150.0;

pub struct WrathPlugin;

impl Plugin for WrathPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnWrathEvent>()
            .add_plugins((spawn::WrathSpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum WrathState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Wrath;

#[derive(Bundle)]
pub struct WrathEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: WrathState,
}

impl Default for WrathEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: WrathState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnWrathEvent {
    pub pos: Vec2,
}
