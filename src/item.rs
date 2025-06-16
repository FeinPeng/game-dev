mod collision;
mod init;
mod pickup;

pub mod sapwn;

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Glue,
    Placebo,
    Schoolbag,
    Wheel,
}

#[derive(Component, Deserialize, Debug, Clone, Copy)]
pub struct ItemEntity {
    pub item: Item,
    pub weight: usize,
}

#[derive(Resource, Deserialize, Debug)]
pub struct ItemPool {
    pub pool: Vec<ItemEntity>,
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            init::InitPlugin,
            sapwn::SpawnPlugin,
            collision::CollisionPlugin,
            pickup::PickUpPlugin,
        ));
    }
}
