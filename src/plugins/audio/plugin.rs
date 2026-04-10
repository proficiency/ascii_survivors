use crate::{events::*, resources::Level};
use bevy::prelude::*;
use bevy_kira_audio::{
    AudioApp, AudioControl,
    prelude::{AudioChannel, AudioPlugin, AudioSource as KiraAudioSource},
};

#[derive(Resource)]
pub struct Music;

#[derive(Resource)]
pub struct Sfx;

#[derive(Clone)]
pub enum AudioChannelType {
    Music,
    Sfx,
}

#[derive(Resource)]
pub struct AudioThemeHandles {
    rest: Handle<KiraAudioSource>,
    combat: Handle<KiraAudioSource>,
}

impl FromWorld for AudioThemeHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            rest: asset_server.load("sfx/loth.ogg"),
            combat: asset_server.load("sfx/harmony.ogg"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum AudioCommand {
    Play {
        audio: &'static str,
        looped: bool,
        volume: Option<f64>,
    },
    PlayHandle {
        audio: Handle<KiraAudioSource>,
        looped: bool,
        volume: Option<f64>,
    },
    Stop,
    Pause,
    Resume,
    SetVolume {
        volume: f64,
    },
}
pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<Music>()
            .add_audio_channel::<Sfx>()
            .init_resource::<AudioThemeHandles>()
            .add_event::<AudioEvent>()
            .add_systems(Update, (play_level_theme, on_audio_event));
    }
}

fn play_level_theme(
    mut level_changed_events: EventReader<LevelChangedEvent>,
    mut audio_events: EventWriter<AudioEvent>,
    theme_handles: Res<AudioThemeHandles>,
) {
    for event in level_changed_events.read() {
        let theme_handle = match event.new_level {
            Level::Rest => theme_handles.rest.clone(),
            _ => theme_handles.combat.clone(),
        };

        // 'flush' the channel before playing the level's theme
        audio_events.write_batch([
            AudioEvent {
                channel: AudioChannelType::Music,
                command: AudioCommand::Stop,
            },
            AudioEvent {
                channel: AudioChannelType::Music,
                command: AudioCommand::PlayHandle {
                    audio: theme_handle,
                    looped: true,
                    volume: Some(0.15),
                },
            },
        ]);
    }
}

fn on_audio_event(
    mut events: EventReader<AudioEvent>,
    music_channel: Res<AudioChannel<Music>>,
    sfx_channel: Res<AudioChannel<Sfx>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        match event.channel {
            AudioChannelType::Music => {
                process_audio_command(&*music_channel, &event.command, &asset_server);
            }
            AudioChannelType::Sfx => {
                process_audio_command(&*sfx_channel, &event.command, &asset_server);
            }
        }
    }
}

fn process_audio_command<T>(
    channel: &AudioChannel<T>,
    command: &AudioCommand,
    asset_server: &AssetServer,
) {
    match command {
        AudioCommand::Play {
            audio,
            looped,
            volume,
        } => {
            info!("[Audio] Playing: {}", audio);

            let mut command = channel.play(asset_server.load(*audio));

            if *looped {
                command.looped();
            }

            if let Some(volume) = volume {
                command.with_volume(*volume);
            }
        }
        AudioCommand::PlayHandle {
            audio,
            looped,
            volume,
        } => {
            let mut command = channel.play(audio.clone());

            if *looped {
                command.looped();
            }

            if let Some(volume) = volume {
                command.with_volume(*volume);
            }
        }
        AudioCommand::Stop => {
            channel.stop();
        }
        AudioCommand::Pause => {
            channel.pause();
        }
        AudioCommand::Resume => {
            channel.resume();
        }
        AudioCommand::SetVolume { volume } => {
            channel.set_volume(*volume);
        }
    }
}
