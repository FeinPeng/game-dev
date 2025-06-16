mod assets;
mod audio;
mod ball;
mod brick;
mod collision_group;
mod enemy;
mod events;
mod item;
mod menu;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{CursorOptions, PresentMode, Window, WindowMode, WindowPlugin};

use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_framepace::{self, FramepacePlugin, FramepaceSettings, Limiter};
use bevy_rapier2d::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Splash,
    Menu,
    Gaming,
    GameOver,
    Win,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: false,
                        canvas: Some("#game-canvas".to_string()),
                        cursor_options: CursorOptions {
                            visible: true,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Splash)
                .load_collection::<GameAssets>(),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(60.0),
        })
        .add_plugins((
            ui::UiPlugin,
            world::WorldPlugin,
            brick::BrickPlugin,
            events::EventPlugin,
            enemy::EnemyPlugin,
            item::ItemPlugin,
            ball::BallPlugin,
            utils::UtilsPlugin,
            menu::MenuPlugin,
            audio::GameAudioPlugin,
        ))
        .run();
}
