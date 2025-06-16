use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{GameAssets, GameState};

use super::Volume;

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), play_menu_bgm)
            .add_systems(OnEnter(GameState::Gaming), play_gaming_bgm)
            .add_systems(
                Update,
                (update_bgm_volumes,).run_if(in_state(GameState::Menu)),
            );
    }
}

fn play_menu_bgm(
    mut audio: ResMut<DynamicAudioChannels>,
    game_assets: Res<GameAssets>,
    volume: Res<Volume>,
) {
    if let Some(channel) = audio.get_channel("game_bgm") {
        channel.pause();
    }
    audio
        .create_channel("menu_bgm")
        .play(game_assets.menu_bgm.clone())
        .with_volume(volume.0 as f64 / 2.0)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .looped();
}

fn play_gaming_bgm(
    mut audio: ResMut<DynamicAudioChannels>,
    game_assets: Res<GameAssets>,
    volume: Res<Volume>,
) {
    if let Some(channel) = audio.get_channel("menu_bgm") {
        channel.pause();
    }
    audio
        .create_channel("gaming_bgm")
        .play(game_assets.gaming_bgm.clone())
        .with_volume(volume.0 as f64 / 2.0)
        .looped();
}

fn update_bgm_volumes(audio: Res<DynamicAudioChannels>, volume: Res<Volume>) {
    if volume.is_changed() {
        if let Some(channel) = audio.get_channel("menu_bgm") {
            channel.set_volume(volume.0 as f64);
        }
        if let Some(channel) = audio.get_channel("gaming_bgm") {
            channel.set_volume(volume.0 as f64);
        }
    }
}
