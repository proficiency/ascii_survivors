pub mod ambient;
pub mod ascii_render;
pub mod audio;
pub mod bootstrap;
pub mod core;
pub mod enemy;
pub mod input;
pub mod interaction;
pub mod player;
pub mod progression;
pub mod rendering;
pub mod scene;
pub mod spell;
pub mod world;

use crate::plugins::{
    ambient::AmbientPlugin, ascii_render::AsciiRenderPlugin, audio::AudioManagerPlugin,
    bootstrap::BootstrapPlugin, core::CorePlugin, enemy::EnemyPlugin, input::InputPlugin,
    interaction::InteractionPlugin, player::PlayerPlugin, progression::ProgressionPlugin,
    rendering::RenderingPlugin, scene::ScenePlugin, spell::SpellPlugin, world::WorldPlugin,
};
use crate::scenes::GameScenesPlugin;
use bevy::prelude::{App, Plugin};

pub struct AsciiSurvivorsPlugins;

impl Plugin for AsciiSurvivorsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BootstrapPlugin,
            CorePlugin,
            AsciiRenderPlugin,
            InputPlugin,
            AudioManagerPlugin,
            SpellPlugin,
            WorldPlugin,
            EnemyPlugin,
        ))
        .add_plugins((InteractionPlugin, RenderingPlugin, AmbientPlugin))
        .add_plugins((
            ProgressionPlugin,
            PlayerPlugin,
            ScenePlugin,
            GameScenesPlugin,
        ));
    }
}
