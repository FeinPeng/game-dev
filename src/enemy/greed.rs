mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const GREED_RADIUS: f32 = 40.0;
pub const GREED_MASS: f32 = 150.0;

pub struct GreedPlugin;

impl Plugin for GreedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnGreedEvent>()
            .add_plugins((spawn::GreedSpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum GreedState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Greed;

#[derive(Bundle)]
pub struct GreedEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: GreedState,
}

impl Default for GreedEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: GreedState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnGreedEvent {
    pub pos: Vec2,
}
