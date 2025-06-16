use std::ops::Neg;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::{ball::Ball, brick::inventory::Inventory, GameAssets, GameState};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<DottedBorderMaterial>::default())
            .add_systems(OnEnter(GameState::Gaming), spawn_inventory)
            .add_systems(
                Update,
                (debug_inventory, update_inventory).run_if(in_state(GameState::Gaming)),
            );
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DottedBorderMaterial {
    #[uniform(0)] // 边框颜色
    border_color: Vec4,
    #[uniform(1)] // 圆点半径（相对比例）
    dot_radius: f32,
    #[uniform(2)] // 圆点间距（相对比例）
    dot_spacing: f32,
    #[uniform(3)] // 边框宽度（相对比例，0~1）
    border_width: f32,
    #[texture(4)] // 中间区域动态纹理
    #[sampler(5)]
    inner_texture: Handle<Image>,
}

// 实现 Material 特质
impl UiMaterial for DottedBorderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/dotted_border.wgsl".into()
    }
}

#[derive(Component)]
pub struct UiInventory;

#[derive(Component)]
pub struct UiInventoryChildNode;

fn spawn_inventory(
    mut commands: Commands,
    mut materials: ResMut<Assets<DottedBorderMaterial>>,
    inventory: Res<Inventory>,
    game_assets: Res<GameAssets>,
) {
    let root = commands
        .spawn((
            Node {
                bottom: Val::Px(1.0),
                right: Val::Px(30.0),
                position_type: PositionType::Absolute,
                // max_width: Val::Px(500.0),
                // max_height: Val::Px(60.0),
                margin: UiRect::all(Val::Px(3.0)),
                flex_direction: FlexDirection::RowReverse,
                flex_wrap: FlexWrap::WrapReverse,
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            UiInventory,
        ))
        .id();
    let virtul_child = commands.spawn(()).id();
    commands.entity(root).add_child(virtul_child);

    for i in 0..inventory.capacity() {
        let child = commands
            .spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                MaterialNode(materials.add(DottedBorderMaterial {
                    border_color: Vec4::new(0.0, 0.0, 0.0, 1.0), // 黑色边框
                    dot_radius: 0.3,                             // 圆点大小（比例）
                    dot_spacing: 0.1,                            // 圆点间距（比例）
                    border_width: 0.05,                          // 边框宽度（占 UI 节点比例）
                    inner_texture: match_ball(inventory.get(i), &game_assets),
                })),
                UiInventoryChildNode,
            ))
            .id();
        commands.entity(root).add_child(child);
    }
}

fn debug_inventory(mut inventory: ResMut<Inventory>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::KeyP) {
        let _ = inventory.push(Ball::Tennis);
    }
    if key.just_pressed(KeyCode::KeyO) {
        inventory.expansion(1);
    }
    if key.just_pressed(KeyCode::KeyI) {
        inventory.pop();
    }
    if key.just_pressed(KeyCode::KeyU) {
        inventory.redusing(1);
    }
}

fn update_inventory(
    mut commands: Commands,
    inventory: Res<Inventory>,
    mut materials: ResMut<Assets<DottedBorderMaterial>>,
    q_mat: Query<&MaterialNode<DottedBorderMaterial>>,
    game_assets: Res<GameAssets>,
    q_ui_inventory: Query<(&Children, Entity), With<UiInventory>>,
) {
    if inventory.is_changed() {
        for (children, entity) in q_ui_inventory.iter() {
            let children_count = children.len() - 1;
            let diff: i32 = inventory.capacity() as i32 - children_count as i32;
            // println!("diff:{}", diff);
            // println!(
            //     "incap: {}, childcount: {}",
            //     inventory.capacity(),
            //     children_count
            // );
            if diff > 0 {
                for _ in 0..diff {
                    let child = commands
                        .spawn((
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                            MaterialNode(materials.add(DottedBorderMaterial {
                                border_color: Vec4::new(0.0, 0.0, 0.0, 1.0), // 黑色边框
                                dot_radius: 0.3,                             // 圆点大小（比例）
                                dot_spacing: 0.1,                            // 圆点间距（比例）
                                border_width: 0.05, // 边框宽度（占 UI 节点比例）
                                inner_texture: match_ball(None, &game_assets),
                            })),
                            UiInventoryChildNode,
                        ))
                        .id();
                    commands.entity(entity).add_child(child);
                }
            }
            if diff < 0 {
                for _ in 0..diff.neg() {
                    if let Some(last_child) = children.last() {
                        commands.entity(entity).remove_children(&[*last_child]);
                        commands.entity(*last_child).despawn();
                    }
                }
            }
            for (index, handle) in q_mat.iter().enumerate() {
                if let Some(materia) = materials.get_mut(handle) {
                    materia.inner_texture = match_ball(inventory.get(index), &game_assets);
                }
            }
        }
    }
}

fn match_ball(ball: Option<Ball>, game_assets: &Res<GameAssets>) -> Handle<Image> {
    if let Some(b) = ball {
        match b {
            Ball::Tennis => game_assets.tennis_texture.clone(),
        }
    } else {
        game_assets.none_texture.clone()
    }
}
