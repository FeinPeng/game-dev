use bevy::prelude::*;

use crate::item::Item;

pub struct ItemCollectionPlugin;

impl Plugin for ItemCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddItemEvent>()
            .init_resource::<ItemCollection>();
    }
}

#[derive(Event)]
pub struct AddItemEvent(pub Item);

#[derive(Resource)]
pub struct ItemCollection(pub Vec<Item>);

impl Default for ItemCollection {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl ItemCollection {
    pub fn add(&mut self, item: Item) {
        self.0.push(item);
    }

    pub fn contains(&self, item: Item) -> bool {
        self.0.contains(&item)
    }
}
