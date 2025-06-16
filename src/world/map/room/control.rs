use bevy::prelude::*;

use crate::{
    enemy::Enemy, utils::anim_sprite::AnimationIndices, GameAssets, GameState, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

use super::{
    create::Arena,
    loading::RoomComponents,
    select::{self, SelectedRooms},
    RoomType,
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (enter_choosing)
                .run_if(in_state(ChooseState::PreChoosing))
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(
            OnEnter(ChooseState::Choosing),
            on_enter_choosing.after(select::select_room),
        );
    }
}

// 此处的选择指的是玩家的选择而不是房间的选择，在创建完房间后将状态从PreChoosing 改为Choosing
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum ChooseState {
    #[default]
    PreChoosing,
    Choosing,
    Ready,
}

pub const ICON_POS_2: [Vec3; 2] = [
    Vec3::new(
        -(WINDOW_WIDTH / 2.0) + 400.0,
        (WINDOW_HEIGHT / 2.0) - 15.0,
        2.0,
    ),
    Vec3::new(
        (WINDOW_WIDTH / 2.0) - 490.0,
        (WINDOW_HEIGHT / 2.0) - 15.0,
        2.0,
    ),
];

fn on_enter_choosing(
    mut commands: Commands,
    mut arena_anim_indices: Query<&mut AnimationIndices, With<Arena>>,
    assets: Res<GameAssets>,
    selected_rooms: Res<SelectedRooms>,
) {
    println!("on enter choosing");
    if selected_rooms.rooms.len() == 0 {
        error!("the num of selected romms is zero");
        panic!();
    }
    for mut indices in arena_anim_indices.iter_mut() {
        indices.last = 3;
        println!("change indices");
    }

    // 生成Icon
    for (index, select_room) in selected_rooms.rooms.iter().enumerate() {
        match select_room.room_type {
            RoomType::Combat => {
                commands.spawn((
                    Sprite::from_image(assets.icon_combat.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
            RoomType::Treasure => {
                commands.spawn((
                    Sprite::from_image(assets.icon_tressure.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
            RoomType::Boss => {
                commands.spawn((
                    Sprite::from_image(assets.icon_boss.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }

            RoomType::PreBoss => {
                commands.spawn((
                    Sprite::from_image(assets.icon_combat.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
            RoomType::PostBoss => {
                commands.spawn((
                    Sprite::from_image(assets.icon_combat.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
            RoomType::Start => {
                commands.spawn((
                    Sprite::from_image(assets.icon_combat.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
            RoomType::Store => {
                commands.spawn((
                    Sprite::from_image(assets.icon_store.clone()),
                    Transform::from_translation(ICON_POS_2.get(index).cloned().unwrap()),
                    RoomComponents,
                ));
            }
        }
    }
}

fn enter_choosing(mut next_state: ResMut<NextState<ChooseState>>, q_enemy: Query<&Enemy>) {
    if q_enemy.iter().len() == 0 {
        // println!("change state to choosing");
        next_state.set(ChooseState::Choosing);
    }
}
