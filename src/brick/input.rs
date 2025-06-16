use bevy::{input::InputSystem, prelude::*};

use crate::GameState;

use super::inventory::Inventory;

// #[derive(Resource, Default)]
// pub struct MouseWorldCoords(pub Vec2);
#[derive(Default)]
pub enum ChooseRoom {
    #[default]
    Idle,
    Left,
    Right,
    Enter,
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub toggle_aim: bool,
    pub shoot: bool,
    pub aim_arrow_rotation: f32,
    pub choose_room: ChooseRoom,
}

#[derive(Event)]
pub struct ToggleAimEvent(pub bool);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                player_movement,
                aim_arrow_rotation,
                toggle_aim,
                shoot,
                choose_room,
            )
                .after(InputSystem)
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<PlayerInput>()
        .add_event::<ToggleAimEvent>();
    }
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, 1.0);
    }

    if keys.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }

    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }

    player_input.move_direction = direction.normalize_or_zero();
}

// pub fn fetch_mouse_world_coords(
//     mut mouse_coords: ResMut<MouseWorldCoords>,
//     window: Single<&Window, With<PrimaryWindow>>,
//     camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     let (camera, camera_transform) = *camera;

//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
//         .map(|ray| ray.unwrap().origin.truncate())
//     {
//         mouse_coords.0 = world_position;
//     }
// }

fn toggle_aim(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
    mut events: EventWriter<ToggleAimEvent>,
    inventory: Res<Inventory>,
) {
    if inventory.index() == 0 {
        return;
    }
    if keys.just_pressed(KeyCode::ShiftLeft) {
        player_input.toggle_aim = !player_input.toggle_aim;
        if player_input.toggle_aim {
            events.send(ToggleAimEvent(true));
        } else {
            events.send(ToggleAimEvent(false));
        }
    }
}

fn shoot(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.shoot = player_input.toggle_aim && keys.just_pressed(KeyCode::ArrowUp);
}

fn aim_arrow_rotation(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut rotation: f32 = 0.0;
    if keys.pressed(KeyCode::ArrowLeft) {
        rotation += 1.0;
    };
    if keys.pressed(KeyCode::ArrowRight) {
        rotation -= 1.0;
    };
    player_input.aim_arrow_rotation = rotation;
}

fn choose_room(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if player_input.toggle_aim {
        return;
    }
    let mut cr = ChooseRoom::Idle;
    if keys.just_pressed(KeyCode::ArrowLeft) {
        cr = ChooseRoom::Left;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        cr = ChooseRoom::Right;
    }
    if keys.just_pressed(KeyCode::Enter) {
        cr = ChooseRoom::Enter;
    }
    // cr = ChooseRoom::Idle;
    player_input.choose_room = cr;
}

// fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
//     *player_input = PlayerInput::default();
// }
