pub mod audio;
pub mod bootstrap;
pub mod input;
pub mod spell;

// todo: put audio/input/etc into their own folders and mod.rs files, only re-exporting what's needed, like events/plugins
use crate::plugins::{
    audio::AudioManagerPlugin, bootstrap::BootstrapPlugin, input::InputPlugin,
    spell::SpellPlugin,
};
use bevy::prelude::{App, Plugin};

pub struct AsciiSurvivorsPlugins;

impl Plugin for AsciiSurvivorsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BootstrapPlugin,
            InputPlugin,
            AudioManagerPlugin,
            SpellPlugin,
        ));
    }
}
