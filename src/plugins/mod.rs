pub mod audio;
pub mod bootstrap;
pub mod input;
pub mod scene;
pub mod schedule;
pub mod spell;

// todo: put audio/input/etc into their own folders and mod.rs files, only re-exporting what's needed, like events/plugins
use crate::plugins::{
    audio::AudioManagerPlugin,
    bootstrap::BootstrapPlugin,
    input::InputPlugin,
    scene::ScenePlugin,
    schedule::SchedulePlugin,
    spell::SpellPlugin,
};
use crate::scenes::GameScenesPlugin;
use bevy::prelude::{App, Plugin};

pub struct AsciiSurvivorsPlugins;

impl Plugin for AsciiSurvivorsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BootstrapPlugin,
            InputPlugin,
            AudioManagerPlugin,
            SpellPlugin,
            SchedulePlugin,
            ScenePlugin,
            GameScenesPlugin,
        ));
    }
}
