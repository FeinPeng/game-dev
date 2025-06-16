use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::{thread_rng, Rng};

mod bgm;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySound>()
            .insert_resource(Volume(1))
            .add_plugins(AudioPlugin)
            .add_plugins((bgm::BgmPlugin,))
            .add_systems(Update, play_sounds);
    }
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[derive(Event)]
pub struct PlaySound {
    pub clip: Handle<AudioSource>,
    pub volume: f64,
    pub playback_rate: f64,
    pub rand_speed_intensity: f64,
    pub repeat: bool,
    pub reverse: bool,
    pub parent: Option<Entity>,
}

impl Default for PlaySound {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            volume: 1.0,
            playback_rate: 1.0,
            rand_speed_intensity: 0.0,
            repeat: false,
            reverse: false,
            parent: None,
        }
    }
}

fn play_sounds(
    mut commands: Commands,
    // audio: Res<Audio>,
    mut audio: ResMut<DynamicAudioChannels>,
    volume: Res<Volume>,
    mut ev_play_sound: EventReader<PlaySound>,
) {
    let mut rng = thread_rng();
    let mut added_sounds: HashSet<Handle<AudioSource>> = HashSet::new();

    for ev in ev_play_sound.read() {
        if added_sounds.contains(&ev.clip) {
            continue;
        }

        println!("sound event");
        added_sounds.insert(ev.clip.clone());

        let speed_offset = if ev.rand_speed_intensity == 0.0 {
            0.0
        } else {
            rng.gen_range(-1.0..1.0) * ev.rand_speed_intensity
        };
        let volume_offset = if ev.parent.is_some() { 0.0 } else { 1.0 };

        let mut audio_channel = audio.create_channel("gaming_bgm").play(ev.clip.clone());
        audio_channel
            .with_volume(ev.volume * volume_offset * volume.0 as f64)
            .with_playback_rate(ev.playback_rate + speed_offset);

        if ev.repeat {
            audio_channel.looped();
        }
        if ev.reverse {
            audio_channel.reverse();
        }

        let audio_instance = audio_channel.handle();

        if let Some(parent) = ev.parent {
            let audio_emitter = commands
                .spawn((
                    Transform::default(),
                    GlobalTransform::default(),
                    SpatialAudioEmitter {
                        instances: vec![audio_instance],
                    },
                ))
                .id();

            match commands.get_entity(parent) {
                Some(mut r) => {
                    r.add_child(audio_emitter);
                }
                None => {
                    warn!("audio parent does not exist");
                }
            };
        };
    }
}
