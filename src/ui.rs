mod cursor;
mod inventory;
mod pressure_bar;

pub mod item_pickup_hint;

use bevy::prelude::*;

pub use cursor::AimAngle;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            pressure_bar::PressureBarPlugin,
            cursor::CursorPlugin,
            inventory::InventoryPlugin,
            item_pickup_hint::ItemPickUpHintPlugin,
        ));
    }
}
