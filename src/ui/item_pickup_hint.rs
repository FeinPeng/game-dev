use bevy::{
    color::palettes::css::{GOLD, WHITE},
    prelude::*,
    text::TextBounds,
};

use crate::{GameAssets, GameState, WINDOW_WIDTH};

pub struct ItemPickUpHintPlugin;

impl Plugin for ItemPickUpHintPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowItemPickupEvent>()
            .insert_resource(PickupQueue::default())
            .add_systems(OnEnter(GameState::Gaming), setup_pickup_ui)
            .add_systems(
                Update,
                (
                    queue_pickup_events,
                    process_pickup_queue,
                    animate_panel_scale,
                    // animate_background,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}

// 道具提示数据
#[derive(Clone)]
pub struct ItemPickupData {
    pub name: String,        // 道具名称
    pub description: String, // 道具描述
}

// 拾取提示触发事件
#[derive(Event)]
pub struct ShowItemPickupEvent(pub ItemPickupData);

// UI组件标记
#[derive(Component)]
struct PickupHintUI;

#[derive(Component)]
struct HintText;

#[derive(Component)]
struct HintTextName;

#[derive(Component)]
struct HintTextDescription;

// 动画状态
#[derive(Component)]
struct HintAnimation {
    timer: Timer,
    state: AnimationState,
}

#[derive(PartialEq)]
enum AnimationState {
    Appearing,
    Visible,
    Disappearing,
}

// 事件队列资源
#[derive(Resource, Default)]
struct PickupQueue(Vec<ItemPickupData>);

fn setup_pickup_ui(mut commands: Commands, game_assets: Res<GameAssets>) {
    let box_size = Vec2::new(WINDOW_WIDTH, 150.0);
    let box_position = Vec2::new(0.0, 250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.25), box_size),
            Transform::from_translation(box_position.extend(4.0)),
            Visibility::Hidden,
            PickupHintUI,
            HintAnimation {
                timer: Timer::from_seconds(0.3, TimerMode::Once),
                state: AnimationState::Disappearing,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(""),
                TextFont {
                    font_size: 32.0,
                    font: game_assets.font_ywgh.clone(),
                    ..default()
                },
                TextLayout::new(JustifyText::Center, LineBreak::AnyCharacter),
                // Wrap text in the rectangle
                // TextBounds::from(box_size),
                Transform::from_translation(Vec3::new(0.0, 20.0, 5.0)),
                HintTextName,
                TextColor(GOLD.into()),
            ));
            parent.spawn((
                Text2d::new(""),
                TextFont {
                    font_size: 24.0,
                    font: game_assets.font_ywgh.clone(),
                    ..default()
                },
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                // TextBounds::from(box_size),
                Transform::from_translation(Vec3::new(0.0, -20.0, 5.0)),
                HintTextDescription,
                TextColor(WHITE.into()),
            ));
        });
}

// 收集拾取事件到队列
fn queue_pickup_events(
    mut events: EventReader<ShowItemPickupEvent>,
    mut queue: ResMut<PickupQueue>,
) {
    for event in events.read() {
        queue.0.push(event.0.clone());
    }
}

// 处理队列中的提示
fn process_pickup_queue(
    mut queue: ResMut<PickupQueue>,
    mut ui_query: Query<(&mut Visibility, &mut HintAnimation), With<PickupHintUI>>,
    mut q_text_name: Query<&mut Text2d, (With<HintTextName>, Without<HintTextDescription>)>,
    mut q_text_description: Query<&mut Text2d, (With<HintTextDescription>, Without<HintTextName>)>,
    time: Res<Time>,
) {
    let Ok((mut visibility, mut anim)) = ui_query.get_single_mut() else {
        return;
    };

    match anim.state {
        AnimationState::Appearing => {
            anim.timer.tick(time.delta());

            if anim.timer.just_finished() {
                anim.state = AnimationState::Visible;
                anim.timer = Timer::from_seconds(2.0, TimerMode::Once);
            }
        }
        AnimationState::Visible => {
            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                anim.state = AnimationState::Disappearing;
                anim.timer = Timer::from_seconds(0.3, TimerMode::Once);
            }
        }
        AnimationState::Disappearing => {
            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                *visibility = Visibility::Hidden;
                if !queue.0.is_empty() {
                    queue.0.remove(0);
                }
            }
        }
    }

    if *visibility == Visibility::Hidden && !queue.0.is_empty() {
        let data = &queue.0[0];

        println!(
            "item name: {}\nitem description: {}",
            data.name, data.description
        );

        // 更新文本
        if let Ok(mut text_name) = q_text_name.get_single_mut() {
            if let Ok(mut text_description) = q_text_description.get_single_mut() {
                **text_name = data.name.clone();
                **text_description = data.description.clone();
            }
        }

        // 启动动画
        *visibility = Visibility::Visible;
        anim.state = AnimationState::Appearing;
        anim.timer = Timer::from_seconds(0.3, TimerMode::Once);
    }
}
// 面板缩放动画
fn animate_panel_scale(
    mut query: Query<&mut Transform, With<PickupHintUI>>,
    anim_query: Query<&HintAnimation>,
) {
    let Ok(anim) = anim_query.get_single() else {
        return;
    };
    let mut transform = query.single_mut();

    match anim.state {
        AnimationState::Appearing => {
            let scale = 0.5 + anim.timer.fraction() * 0.5;
            transform.scale = Vec3::splat(scale);
        }
        AnimationState::Disappearing => {
            let scale = 1.0 - anim.timer.fraction();
            transform.scale = Vec3::splat(scale);
        }
        _ => {}
    }
}

// // 背景渐变动画
// fn animate_background(
//     mut query: Query<&mut Sprite, With<PickupHintUI>>,
//     anim_query: Query<&HintAnimation>,
// ) {
//     let Ok(anim) = anim_query.get_single() else {
//         return;
//     };
//     let mut sprite = query.single_mut();

//     match anim.state {
//         AnimationState::Appearing => {
//             sprite.color.set_alpha(anim.timer.fraction() * 0.8);
//         }
//         AnimationState::Disappearing => {
//             sprite.color.set_alpha(0.8 - (anim.timer.fraction() * 0.8));
//         }
//         _ => {}
//     }
// }
