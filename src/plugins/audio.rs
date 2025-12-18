use crate::{Level, events::*};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct Music;

#[derive(Resource)]
pub struct Sfx;

#[derive(Clone)]
pub enum AudioChannelType {
    Music,
    Sfx,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum AudioCommand {
    Play {
        audio: &'static str,
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
            .add_event::<AudioEvent>()
            .add_systems(Update, (play_level_theme, on_audio_event));
    }
}

fn play_level_theme(
    mut level_changed_events: EventReader<LevelChangedEvent>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    for event in level_changed_events.read() {
        let theme_path = match event.new_level {
            Level::Rest => "sfx/loth.ogg",
            _ => "sfx/harmony.ogg",
        };

        // 'flush' the channel before playing the level's theme
        audio_events.write_batch([
            AudioEvent {
                channel: AudioChannelType::Music,
                command: AudioCommand::Stop,
            },
            AudioEvent {
                channel: AudioChannelType::Music,
                command: AudioCommand::Play {
                    audio: theme_path,
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
