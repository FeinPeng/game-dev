use std::fs::File;

use bevy::prelude::*;
use ron::{self, de::from_reader};

use super::ItemPool;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_rooms);
    }
}

fn init_rooms(mut commands: Commands) {
    let file = File::open("assets/items/items.ron").expect("Failed to open items.ron");
    let item_pool: ItemPool = from_reader(file).expect("Unable to load items.ron");
    commands.insert_resource(item_pool);
}
