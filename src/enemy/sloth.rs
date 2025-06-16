mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const SLOTH_WIDTH: f32 = 150.0;
pub const SLOTH_HEIGHT: f32 = 45.0;
pub const SLOTH_MASS: f32 = 150.0;

pub struct SlothPlugin;

impl Plugin for SlothPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnSlothEvent>()
            .add_plugins((spawn::SlothSpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum SlothState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Sloth;

#[derive(Bundle)]
pub struct SlothEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: SlothState,
}

impl Default for SlothEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: SlothState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnSlothEvent {
    pub pos: Vec2,
}
