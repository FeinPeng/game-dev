pub mod control;
pub mod create;
pub mod loading;
pub mod select;

mod init;

use bevy::prelude::*;
use control::ChooseState;
use serde::Deserialize;

use crate::enemy::Enemy;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum RoomType {
    Combat,
    Treasure,
    Boss,
    PreBoss,
    PostBoss,
    Start,
    Store,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct DeserVec2 {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for DeserVec2 {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Room {
    pub room_type: RoomType,
    pub num_exits: usize,
    pub arena: usize,
    pub encounters: Option<Vec<Encounter>>,
    pub weight: usize,
    pub force_select_depth_min: Option<usize>,
    pub force_select_depth_max: Option<usize>,
    pub is_select: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Encounter {
    pub enemys: Vec<EnemyEntity>,
    pub weight: usize,
    pub is_select: bool,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct EnemyEntity {
    pub enemy_type: Enemy,
    pub position: DeserVec2,
}

#[derive(Resource, Deserialize, Debug)]
pub struct Rooms {
    pub rooms: Vec<Room>,
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ChooseState>().add_plugins((
            init::InitPlugin,
            select::SelectPlugin,
            loading::LoadingPlugin,
            control::ControlPlugin,
        ));
    }
}
