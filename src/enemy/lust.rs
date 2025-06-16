mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const LUST_WIDTH: f32 = 70.0;
pub const LUST_HEIGHT: f32 = 70.0;
pub const LUST_MASS: f32 = 150.0;

pub struct LustPlugin;

impl Plugin for LustPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnLustEvent>()
            .add_plugins((spawn::LustSpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum LustState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Lust;

#[derive(Bundle)]
pub struct LustEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: LustState,
}

impl Default for LustEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: LustState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnLustEvent {
    pub pos: Vec2,
}
