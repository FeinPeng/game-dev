mod collision;
mod spawn;

use bevy::prelude::*;

use crate::{brick::stats::Pressure, events::Damage};

pub const GLUTTONY_WIDTH: f32 = 170.0;
pub const GLUTTONY_HEIGHT: f32 = 160.0;
pub const GLUTTONY_MASS: f32 = 150.0;

pub struct GluttonyPlugin;

impl Plugin for GluttonyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<SpawnGluttonyEvent>()
            .add_plugins((spawn::GluttonySpawnPlugin, collision::CollisionPlugin));
    }
}

#[derive(Default, PartialEq, Clone, Copy, Component)]
pub enum GluttonyState {
    #[default]
    Idling,
    Dying,
}

#[derive(Component)]
pub struct Gluttony;

#[derive(Bundle)]
pub struct GluttonyEnemy {
    pub pressure: Pressure,
    pub damage: Damage,
    pub state: GluttonyState,
}

impl Default for GluttonyEnemy {
    fn default() -> Self {
        Self {
            pressure: Pressure {
                current: 0.0,
                max: 50.0,
            },
            damage: Damage { value: 10.0 },
            state: GluttonyState::Idling,
        }
    }
}

#[derive(Event)]
pub struct SpawnGluttonyEvent {
    pub pos: Vec2,
}
