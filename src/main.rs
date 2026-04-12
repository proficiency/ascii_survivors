mod debug;
mod effects;
mod events;
mod maps;
mod objects;
mod plugins;
mod resources;
mod scenes;
mod spells;
mod upgrades;

use crate::{debug::DebugPlugins, plugins::AsciiSurvivorsPlugins};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            AsciiSurvivorsPlugins,
            #[cfg(debug_assertions)]
            DebugPlugins,
        ))
        .run();
}
