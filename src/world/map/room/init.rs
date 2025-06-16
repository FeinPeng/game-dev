use std::fs::File;

use bevy::prelude::*;
use ron::{self, de::from_reader};

use super::Rooms;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_rooms);
    }
}

fn init_rooms(mut commands: Commands) {
    let file = File::open("assets/rooms/rooms.ron").expect("Failed to open rooms.ron");
    let rooms: Rooms = from_reader(file).expect("Unable to load rooms.ron");
    commands.insert_resource(rooms);
}
