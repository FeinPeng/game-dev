pub mod envy;
pub mod gluttony;
pub mod greed;
pub mod lust;
pub mod pride;
pub mod sloth;
pub mod wrath;

use bevy::prelude::*;
use serde::Deserialize;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            envy::EnvyPlugin,
            gluttony::GluttonyPlugin,
            greed::GreedPlugin,
            lust::LustPlugin,
            pride::PridePlugin,
            sloth::SlothPlugin,
            wrath::WrathPlugin,
        ));
    }
}

#[derive(Component, Deserialize, Debug, Clone, Copy)]
pub enum Enemy {
    Sloth,
    BossA,
    Envy,
    Gluttony,
    Greed,
    Pride,
    Wrath,
    Lust,
}
