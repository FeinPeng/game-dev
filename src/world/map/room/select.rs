use bevy::prelude::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::brick::stats::BrickStats;

use super::{ChooseState, Encounter, Room, RoomType, Rooms};

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedRooms>()
            .add_systems(OnEnter(ChooseState::Choosing), select_room);
    }
}

#[derive(Resource, Debug)]
pub struct SelectedRooms {
    pub rooms: Vec<SelectedRoom>,
    pub index: usize,
}

impl Default for SelectedRooms {
    fn default() -> Self {
        Self {
            rooms: Vec::new(),
            index: 0,
        }
    }
}

impl SelectedRooms {
    pub fn increse_index(&mut self) {
        let mut i = self.index as i32;
        let room_len = self.rooms.len() as i32 - 1;
        i += 1;
        if i > room_len {
            i = 0;
        }
        self.index = i as usize;
    }

    pub fn decrese_index(&mut self) {
        let mut i = self.index as i32;
        let room_len = self.rooms.len() as i32 - 1;
        i -= 1;
        if i < 0 {
            i = room_len;
        }
        self.index = i as usize;
    }
}

#[derive(Debug)]
pub struct SelectedRoom {
    pub room_type: RoomType,
    pub num_exits: usize,
    pub arena: usize,
    pub encounter: Option<Encounter>,
}

pub fn select_room(
    mut selected_rooms: ResMut<SelectedRooms>,
    mut rooms: ResMut<Rooms>,
    brick_state: Res<BrickStats>,
) {
    println!("select room");
    let mut force = false;
    let mut rooms = rooms
        .rooms
        .iter_mut()
        .filter(|room| !room.is_select)
        .filter(|room| {
            if let Some(min) = room.force_select_depth_min {
                if let Some(max) = room.force_select_depth_max {
                    if brick_state.current_room == max {
                        force = true;
                        return true;
                    } else if brick_state.current_room >= min && brick_state.current_room < max {
                        return true;
                    }
                }
            }
            if force {
                return false;
            } else {
                return true;
            }
        })
        .collect::<Vec<&mut Room>>();
    if force && rooms.len() > 1 {
        panic!("The num of force selceted room is more than one");
    }

    // println!("rooms: {:#?}", rooms);

    for _ in 0..brick_state.num_exits {
        // 根据权重来随机选择房间
        let mut weights = rooms.iter().map(|room| room.weight).collect::<Vec<usize>>();
        let mut dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        let select_room = rooms.remove(dist.sample(&mut rng));
        select_room.is_select = true;
        // println!("select room: {:#?}", select_room.room_type);

        // 随机选择遭遇,没有的话为None
        let mut encounter = None;
        if let Some(ref mut encounters) = select_room.encounters {
            weights = encounters
                .iter()
                .map(|enc| enc.weight)
                .collect::<Vec<usize>>();
            dist = WeightedIndex::new(&weights).unwrap();
            let enc = encounters.get_mut(dist.sample(&mut rng)).unwrap();
            enc.is_select = true;
            encounter = Some(enc.clone())
        }

        selected_rooms.rooms.push(SelectedRoom {
            room_type: select_room.room_type,
            num_exits: select_room.num_exits,
            arena: select_room.arena,
            encounter,
        });

        for room in rooms.iter_mut() {
            if let Some(_) = room.force_select_depth_min {
                if let Some(_) = room.force_select_depth_max {
                    room.weight += 100_usize;
                }
            }
        }
    }
    // println!("selected rooms: {:#?}", selected_rooms);
}
