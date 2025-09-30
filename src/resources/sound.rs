use anyhow::*;
use bevy::prelude::Resource;
use kira::{
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Tween, sound::static_sound::*,
};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Resource)]
pub struct SoundManager {
    manager: AudioManager<DefaultBackend>,
    sounds: HashMap<String, StaticSoundData>,
    theme_handle: Option<kira::sound::static_sound::StaticSoundHandle>,
}

impl SoundManager {
    pub fn new(sound_path: PathBuf) -> Result<Self> {
        let sound_files = std::fs::read_dir(&sound_path)
            .unwrap_or_else(|_| {
                std::fs::read_dir("./assets/sfx/").expect("Failed to read default sound directory")
            }) // fallback to default path
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let ext = path.extension()?.to_str()?;
                if ext == "wav" || ext == "ogg" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        let mut sounds = HashMap::<String, StaticSoundData>::with_capacity(sound_files.len());
        for file_path in sound_files {
            let file_path = file_path.to_string_lossy().replace("\\", "/");

            sounds.insert(
                file_path.clone(),
                StaticSoundData::from_file(file_path.clone()).map_err(|e| anyhow::anyhow!(e))?,
            );

            sounds.insert(
                file_path.clone().to_string().to_string(),
                StaticSoundData::from_file(&file_path).map_err(|e| anyhow::anyhow!(e))?,
            );

            sounds.insert(
                file_path.to_string(),
                StaticSoundData::from_file(file_path).map_err(|e| anyhow::anyhow!(e))?,
            );
        }

        // todo: implement 'tracing' crate
        println!("[SoundManager] cached {} audio files", sounds.len());

        Ok(Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?,
            sounds,
            theme_handle: None,
        })
    }

    pub fn play_sound(&mut self, file_path: PathBuf, volume: f32) -> Result<StaticSoundHandle> {
        let sound = self
            .sounds
            .get(file_path.to_str().unwrap())
            .ok_or_else(|| anyhow::anyhow!("sound not found: {}", file_path.to_str().unwrap()))?
            .volume(Decibels(volume));

        self.manager.play(sound).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn play_theme(&mut self, volume: f32) -> Result<()> {
        let sound = self
            .sounds
            .get("./assets/sfx/harmony.ogg")
            .ok_or_else(|| anyhow::anyhow!("theme not found"))?
            .volume(Decibels(volume));

        let handle = self.manager.play(sound).map_err(|e| anyhow::anyhow!(e))?;
        self.theme_handle = Some(handle);
        Ok(())
    }

    pub fn stop_theme(&mut self) {
        if let Some(mut handle) = self.theme_handle.take() {
            handle.stop(Tween::default());
        }
    }
}
