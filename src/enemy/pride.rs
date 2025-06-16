mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const PRIDE_WIDTH: f32 = 90.0;
pub const PRIDE_HEIGHT: f32 = 70.0;
pub const PRIDE_MASS: f32 = 150.0;

pub struct PridePlugin;

impl Plugin for PridePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnPrideEvent>()
            .add_plugins((spawn::PrideSpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum PrideState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Pride;

#[derive(Bundle)]
pub struct PrideEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: PrideState,
}

impl Default for PrideEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: PrideState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnPrideEvent {
    pub pos: Vec2,
}
