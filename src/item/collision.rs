use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::brick::item_collection::{AddItemEvent, ItemCollection};

use super::{sapwn::SensorItem, Item};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_sensor_door);
    }
}

pub fn handle_sensor_door(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut item_collection: ResMut<ItemCollection>,
    mut add_item_events_writer: EventWriter<AddItemEvent>,
    q_sensor: Query<(), (With<Sensor>, With<SensorItem>)>,
    q_parent: Query<&Parent>,
    q_item: Query<&Item>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = event {
            if q_sensor.contains(*entity_a) {
                // a 为item b 为 brick
                let parent_item = q_parent.get(*entity_a).unwrap().get();
                let item = q_item.get(parent_item).unwrap();
                item_collection.add(*item);
                add_item_events_writer.send(AddItemEvent(*item));
                commands.entity(parent_item).despawn_recursive();
            }
            if q_sensor.contains(*entity_b) {
                // b 为item a 为 brick
                let parent_item = q_parent.get(*entity_b).unwrap().get();
                let item = q_item.get(parent_item).unwrap();
                item_collection.add(*item);
                add_item_events_writer.send(AddItemEvent(*item));
                commands.entity(parent_item).despawn_recursive();
            }
        }
    }
}
