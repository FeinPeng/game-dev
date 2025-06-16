use bevy::prelude::*;
use bevy_rapier2d::prelude::Friction;

use crate::{
    ball::Ball,
    brick::{inventory::Inventory, item_collection::AddItemEvent, stats::Pressure, Brick, Speed},
    ui::item_pickup_hint::{ItemPickupData, ShowItemPickupEvent},
    GameState,
};

pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pickup).run_if(in_state(GameState::Gaming)));
    }
}

fn pickup(
    mut events: EventWriter<ShowItemPickupEvent>,
    mut add_item_events_reader: EventReader<AddItemEvent>,
    mut q_pressure: Query<&mut Pressure, With<Brick>>,
    mut inventory: ResMut<Inventory>,
    mut q_fraction: Query<&mut Friction, With<Brick>>,
    mut q_speed: Query<&mut Speed, With<Brick>>,
) {
    for &AddItemEvent(item) in add_item_events_reader.read() {
        match item {
            super::Item::Glue => {
                // println!("pickup ");
                events.send(ShowItemPickupEvent(ItemPickupData {
                    name: "胶水".into(),
                    description: "增加摩檫力".into(),
                }));

                if let Ok(mut fric) = q_fraction.get_single_mut() {
                    fric.coefficient += 20.0;
                }
            }
            super::Item::Placebo => {
                // println!("pickup ");
                events.send(ShowItemPickupEvent(ItemPickupData {
                    name: "安慰剂".into(),
                    description: "减少压力提升压力上限".into(),
                }));

                if let Ok(mut pressure) = q_pressure.get_single_mut() {
                    let mut current = pressure.current - 20.0;
                    if current < 0.0 {
                        current = 0.0
                    }
                    pressure.current = current;
                    pressure.max += 20.0;
                }
            }
            super::Item::Schoolbag => {
                // println!("pickup ");
                events.send(ShowItemPickupEvent(ItemPickupData {
                    name: "书包".into(),
                    description: "扩容！！！".into(),
                }));

                inventory.expansion(2);

                if let Ok(_) = inventory.push(Ball::Tennis) {
                    if let Ok(_) = inventory.push(Ball::Tennis) {}
                }
            }
            super::Item::Wheel => {
                // println!("pickup ");
                events.send(ShowItemPickupEvent(ItemPickupData {
                    name: "轮子".into(),
                    description: "速度提升".into(),
                }));

                if let Ok(mut speed) = q_speed.get_single_mut() {
                    speed.0 += 20.0;
                }
            }
        }
    }
}
