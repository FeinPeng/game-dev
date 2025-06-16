use bevy::prelude::*;

use crate::{
    brick::stats::Pressure, brick::Brick, GameAssets, GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
};

// 压力条UI标记组件
#[derive(Component)]
struct PressureBar;

// 压力条前景（实际血量显示部分）
#[derive(Component)]
struct PressureBarForeground;

#[derive(Component)]
struct PressureText;

pub struct PressureBarPlugin;

impl Plugin for PressureBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_pressure_bar)
            .add_systems(
                Update,
                (update_pressure_bar,).run_if(in_state(GameState::Gaming)),
            );
    }
}

fn spawn_pressure_bar(mut commands: Commands, assets: Res<GameAssets>) {
    // 角色压力条
    commands
        // 压力条背景
        .spawn((
            Sprite {
                color: Color::srgb(0.25, 0.25, 0.25),
                custom_size: Some(Vec2::new(105.0, 20.0)),
                ..default()
            },
            // Transform::from_xyz(10.0, WINDOW_HEIGHT - 50.0, 2.0),
            Transform::from_xyz(
                (-WINDOW_WIDTH / 2.0) + 85.0,
                (-WINDOW_HEIGHT / 2.0) + 20.0,
                2.0,
            ),
            PressureBar,
        ))
        .with_children(|parent| {
            // 压力条前景
            parent.spawn((
                Sprite {
                    color: Color::srgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(100.0, 15.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 2.0),
                PressureBarForeground,
            ));

            parent.spawn((
                Text2d::new("0/100"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(Color::BLACK),
                // Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 50.0, -WINDOW_HEIGHT / 2.0 + 30.0, 2.0),
                Transform::from_xyz(0.0, 20.0, 2.0),
                PressureText,
            ));
        });
}

fn update_pressure_bar(
    pressure_query: Query<&Pressure, (Changed<Pressure>, With<Brick>)>,

    mut foreground_query: Query<&mut Transform, With<PressureBarForeground>>,
    mut pressure_bar_foreground_sprite: Single<&mut Sprite, With<PressureBarForeground>>,
    mut presssure_text_query: Query<(&mut Text2d, &mut TextColor), With<PressureText>>,
) {
    for pressure in pressure_query.iter() {
        let pressure_ratio = pressure.current / pressure.max;

        for mut transform in foreground_query.iter_mut() {
            // 调整压力条的长度和位置（保持左对齐）
            transform.scale.x = pressure_ratio;
            transform.translation.x = -(1.0 - pressure_ratio) * 50.0;
        }

        // 颜色渐变
        let color = Color::srgb(1.0, 1.0 - pressure_ratio, 1.0 - pressure_ratio);
        pressure_bar_foreground_sprite.color = color;

        for (mut text, mut text_color) in presssure_text_query.iter_mut() {
            text.0 = format!("{}/{}", pressure.current as i32, pressure.max as i32);
            text_color.0 = color;
        }
    }
} // 测试伤害系统

// fn damage_test(mut pressure_query: Query<&mut Pressure>, keyboard: Res<ButtonInput<KeyCode>>) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         for mut pressure in pressure_query.iter_mut() {
//             pressure.current = (pressure.current + 10.0).min(pressure.max);
//             info!("当前压力条:{}/{}", pressure.current, pressure.max);
//         }
//     }
//     if keyboard.just_pressed(KeyCode::KeyC) {
//         for mut pressure in pressure_query.iter_mut() {
//             pressure.current = (pressure.current - 10.0).max(0.0);
//             info!("当前压力条:{}/{}", pressure.current, pressure.max);
//         }
//     }
// }
