use bevy::prelude::{App, Plugin};

mod performance_overlay;
pub use performance_overlay::*;

pub struct DebugPlugins;

impl Plugin for DebugPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerformanceOverlayPlugin);
    }
}
