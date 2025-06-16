use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{
    brick::{input::PlayerInput, Brick},
    GameAssets, GameState,
};

pub const RADIUS: f32 = 150.0;
pub const ANGLE_VELOCITY: f32 = 2.0;

#[derive(Resource)]
pub struct AimAngle(pub f32);

impl Default for AimAngle {
    fn default() -> Self {
        Self(FRAC_PI_2)
    }
}

#[derive(Component)]
pub struct Cursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_cursor)
            .add_systems(Update, (update_cursor,).run_if(in_state(GameState::Gaming)))
            .init_resource::<AimAngle>();
    }
}

fn spawn_cursor(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(assets.cursor_texture.clone()),
        Transform::from_xyz(100.0, 0.0, 2.0),
        Visibility::Hidden,
        Cursor,
    ));
}

fn update_cursor(
    mut cursor: Single<(&mut Transform, &mut Visibility, &mut Sprite), With<Cursor>>,
    brick_transform: Single<&Transform, (With<Brick>, Without<Cursor>)>,
    mut player_input: ResMut<PlayerInput>,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut angle: ResMut<AimAngle>,
) {
    if !player_input.toggle_aim {
        *cursor.1 = Visibility::Hidden;
        *angle = AimAngle::default();
        return;
    }
    *cursor.1 = Visibility::Visible;

    angle.0 += player_input.aim_arrow_rotation * ANGLE_VELOCITY * time.delta_secs();
    angle.0 = angle.0.clamp(15.0_f32.to_radians(), 165.0_f32.to_radians());
    // 圆参数方程求箭头位置
    let x = brick_transform.translation.x + angle.0.cos() * RADIUS;
    let y = brick_transform.translation.y + angle.0.sin() * RADIUS;
    let target = Vec2::new(x, y);

    // 调整cursor
    cursor.0.rotation = Quat::from_rotation_z(angle.0 - FRAC_PI_2);
    cursor.0.translation = target.extend(2.0);

    // 如果cursor 在brick的下边，则改变image且不可发射
    // 使用angle.0.clamp 之后这段其实可以删掉
    if !(angle.0 > 15.0_f32.to_radians() && angle.0 < 165.0_f32.to_radians()) {
        player_input.shoot = false;
        cursor.2.image = assets.cursor_invalid_texture.clone();
    } else {
        cursor.2.image = assets.cursor_texture.clone();
    }
}
