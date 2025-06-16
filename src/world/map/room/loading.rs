use bevy::{ecs::system::SystemId, prelude::*, state::commands};

use crate::{
    brick::{stats::BrickStats, Brick},
    enemy::{
        envy::SpawnEnvyEvent, gluttony::SpawnGluttonyEvent, greed::SpawnGreedEvent,
        lust::SpawnLustEvent, pride::SpawnPrideEvent, sloth::SpawnSlothEvent,
        wrath::SpawnWrathEvent,
    },
    world::map::pipelines_readdy::{PipelinesReady, PipelinesReadyPlugin},
    GameState,
};

use super::{create, select::SelectedRooms, ChooseState, Enemy, RoomType};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PipelinesReadyPlugin)
            .add_systems(Startup, load_loading_screen)
            .add_systems(OnEnter(LoadingState::FadeOut), init_room_data)
            .add_systems(
                OnExit(LoadingState::FadeOut),
                (unload_current_room, load_next_room),
            )
            .add_systems(Update, fade_control.run_if(in_state(LoadingState::FadeOut)))
            .add_systems(Update, fade_control.run_if(in_state(LoadingState::FadeIn)))
            .add_systems(
                Update,
                update_loading_data.run_if(in_state(LoadingState::Loading)),
            )
            .init_state::<LoadingState>()
            .insert_resource(LoadingData::new(0));
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LoadingState {
    #[default]
    Ready,
    Loading,
    FadeOut, // 淡出状态（场景切换前）
    FadeIn,  // 淡入状态（场景切换后）
}

// 淡入淡出组件
#[derive(Component)]
struct FadeTransition {
    timer: Timer,
}

// A resource that holds the current loading data.
#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub loading_assets: Vec<UntypedHandle>,
    // Number of frames that everything needs to be ready for.
    // This is to prevent going into the fully loaded state in instances
    // where there might be a some frames between certain loading/pipelines action.
    pub confirmation_frames_target: usize,
    // Current number of confirmation frames.
    pub confirmation_frames_count: usize,
}

impl LoadingData {
    fn new(confirmation_frames_target: usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

// Monitors current loading status of assets.
fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut next_loading_state: ResMut<NextState<LoadingState>>,
    asset_server: Res<AssetServer>,
    pipelines_ready: Res<PipelinesReady>,
) {
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        // If we are still loading assets / pipelines are not fully compiled,
        // we reset the confirmation frame count.
        loading_data.confirmation_frames_count = 0;

        loading_data.loading_assets.retain(|asset| {
            asset_server
                .get_recursive_dependency_load_state(asset)
                .is_none_or(|state| !state.is_loaded())
        });

        // If there are no more assets being monitored, and pipelines
        // are compiled, then start counting confirmation frames.
        // Once enough confirmations have passed, everything will be
        // considered to be fully loaded.
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count >= loading_data.confirmation_frames_target {
            println!("finish loading data");
            next_loading_state.set(LoadingState::FadeIn);
        }
    }
}

#[derive(Resource)]
struct RoomData {
    load_room_id: SystemId,
    load_room_enemys_id: SystemId,
}

fn init_room_data(mut commands: Commands, selected_rooms: Res<SelectedRooms>) {
    let room_data = RoomData {
        load_room_id: match selected_rooms
            .rooms
            .get(selected_rooms.index)
            .unwrap()
            .room_type
        {
            RoomType::Boss => commands.register_system(create::boss),
            RoomType::Combat => commands.register_system(create::combat),
            RoomType::PostBoss => commands.register_system(create::post_boss),
            RoomType::PreBoss => commands.register_system(create::pre_boss),
            RoomType::Start => commands.register_system(create::start),
            RoomType::Treasure => commands.register_system(create::treasure),
            RoomType::Store => commands.register_system(create::store),
        },
        load_room_enemys_id: commands.register_system(load_enemys),
    };
    commands.insert_resource(room_data);
}

// Marker component for easier deletion of entities.
#[derive(Component)]
pub struct RoomComponents;

// Removes all currently loaded room assets from the game World.
fn unload_current_room(
    mut commands: Commands,
    // mut next_loading_state: ResMut<NextState<LoadingState>>,
    entities: Query<Entity, With<RoomComponents>>,
) {
    // *loading_state = LoadingState::Loading;
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn load_next_room(mut commands: Commands, room_data: Res<RoomData>) {
    commands.run_system(room_data.load_room_id);
    commands.run_system(room_data.load_room_enemys_id);
}

fn load_enemys(
    selected_rooms: Res<SelectedRooms>,
    mut spawn_sloth_events_writer: EventWriter<SpawnSlothEvent>,
    mut spawn_envy_events_writer: EventWriter<SpawnEnvyEvent>,
    mut spawn_gluttony_events_writer: EventWriter<SpawnGluttonyEvent>,
    mut spawn_greed_events_writer: EventWriter<SpawnGreedEvent>,
    mut spawn_pride_events_writer: EventWriter<SpawnPrideEvent>,
    mut spawn_wrath_events_writer: EventWriter<SpawnWrathEvent>,
    mut spawn_lust_events_writer: EventWriter<SpawnLustEvent>,
) {
    if let Some(select_room) = selected_rooms.rooms.get(selected_rooms.index) {
        if let Some(enconter) = &select_room.encounter {
            for enemy_entity in enconter.enemys.iter() {
                match enemy_entity.enemy_type {
                    Enemy::Sloth => {
                        spawn_sloth_events_writer.send(SpawnSlothEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::BossA => {
                        spawn_sloth_events_writer.send(SpawnSlothEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Envy => {
                        spawn_envy_events_writer.send(SpawnEnvyEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Gluttony => {
                        spawn_gluttony_events_writer.send(SpawnGluttonyEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Greed => {
                        spawn_greed_events_writer.send(SpawnGreedEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Pride => {
                        spawn_pride_events_writer.send(SpawnPrideEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Wrath => {
                        spawn_wrath_events_writer.send(SpawnWrathEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                    Enemy::Lust => {
                        spawn_lust_events_writer.send(SpawnLustEvent {
                            pos: enemy_entity.position.into(),
                        });
                    }
                }
            }
        }
    }
}

// Marker tag for loading screen components.
#[derive(Component)]
struct LoadingScreen;

// Spawns the necessary components for the loading screen.
fn load_loading_screen(mut commands: Commands) {
    // Spawn the UI that will make up the loading screen.
    commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        FadeTransition {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        BackgroundColor(Color::Srgba(Srgba::new(0.0, 0.0, 0.0, 0.0))),
        LoadingScreen,
    ));
}

fn fade_control(
    time: Res<Time>,
    mut selected_rooms: ResMut<SelectedRooms>,
    mut query: Query<(&mut FadeTransition, &mut BackgroundColor)>,
    loading_state: Res<State<LoadingState>>,
    mut next_loading_state: ResMut<NextState<LoadingState>>,
    mut next_choose_state: ResMut<NextState<ChooseState>>,
    mut brick_transform: Single<&mut Transform, With<Brick>>,
    mut brick_stats: ResMut<BrickStats>,
) {
    for (mut transition, mut color) in &mut query {
        transition.timer.tick(time.delta());

        let alpha = match **loading_state {
            LoadingState::FadeIn => 1.0 - transition.timer.fraction(),
            LoadingState::FadeOut => transition.timer.fraction(),
            _ => unreachable!("match error loading_state"),
        };
        color.0.set_alpha(alpha);

        // 过渡完成时切换状态
        if transition.timer.finished() {
            match **loading_state {
                LoadingState::FadeOut => {
                    // 淡出完成后开始加载场景
                    println!("finish fade out");
                    next_loading_state.set(LoadingState::FadeIn);
                    brick_transform.translation = Vec3::new(0.0, -250.0, 1.0);
                    transition.timer.reset();
                }
                LoadingState::FadeIn => {
                    // 淡入完成后恢复正常状态
                    println!("finish fade in");
                    next_choose_state.set(ChooseState::PreChoosing);
                    next_loading_state.set(LoadingState::Ready);
                    brick_stats.current_room += 1;
                    *selected_rooms = SelectedRooms::default();
                    transition.timer.reset();
                }
                _ => unreachable!("match error loading_state"),
            }
        }
    }
}
