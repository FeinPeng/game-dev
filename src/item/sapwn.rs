use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::{
    collision_group::{GROUP_BRICK, GROUP_ITEM},
    world::map::room::loading::RoomComponents,
    GameAssets, GameState,
};

use super::{Item, ItemPool};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnItemEvent>().add_systems(
            Update,
            handle_spawn_item_event.run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub pos: Vec2,
}

fn handle_spawn_item_event(
    mut commands: Commands,
    mut item_pool: ResMut<ItemPool>,
    mut event_reader: EventReader<SpawnItemEvent>,
    game_assets: Res<GameAssets>,
) {
    for &SpawnItemEvent { pos } in event_reader.read() {
        if item_pool.pool.len() != 0 {
            let weights = item_pool
                .pool
                .iter()
                .map(|item| item.weight)
                .collect::<Vec<usize>>();
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();
            let select_item = item_pool.pool.remove(dist.sample(&mut rng));
            sapwn_item(&mut commands, pos, select_item.item, &game_assets);
        } else {
            sapwn_item(&mut commands, pos, Item::Schoolbag, &game_assets);
        }
    }
}

#[derive(Component)]
pub struct SensorItem;

pub fn sapwn_item(commands: &mut Commands, pos: Vec2, item: Item, game_assets: &Res<GameAssets>) {
    let image = match item {
        Item::Glue => game_assets.item_glue.clone(),
        Item::Placebo => game_assets.item_placebo.clone(),
        Item::Schoolbag => game_assets.item_schoolbag.clone(),
        Item::Wheel => game_assets.item_wheel.clone(),
    };
    let entity = commands
        .spawn((
            Sprite::from_image(image),
            RigidBody::KinematicVelocityBased,
            Transform::from_translation(pos.extend(1.0)),
            item,
            RoomComponents,
        ))
        .id();
    let collider = commands
        .spawn((
            Collider::ball(25.0),
            SensorItem,
            Sensor,
            CollisionGroups::new(GROUP_ITEM, GROUP_BRICK),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();
    commands.entity(entity).add_child(collider);
}
