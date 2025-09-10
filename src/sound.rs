use anyhow::*;
use kira::{AudioManager, AudioManagerSettings, Decibels, DefaultBackend, sound::static_sound::*};
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) struct SoundManager {
    manager: AudioManager<DefaultBackend>,
    sounds: HashMap<String, StaticSoundData>,
}

impl SoundManager {
    pub fn new(sound_path: PathBuf) -> Result<Self> {
        let sound_files = std::fs::read_dir(&sound_path)?
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
        for file_path in &sound_files {
            sounds.insert(file_path.to_str().unwrap().to_string(),StaticSoundData::from_file(file_path).map_err(|e| anyhow::anyhow!(e))?);
        }

        // todo: implement 'tracing' crate
        println!("[SoundManager] cached {} audio files", sounds.len());

        Ok(Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?,
            sounds,
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
}
