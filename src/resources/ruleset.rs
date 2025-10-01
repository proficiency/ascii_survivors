use bevy::prelude::*;

#[derive(Resource)]
pub struct Ruleset {
    pub portal_spawn_time: f32,
}

impl Default for Ruleset {
    fn default() -> Self {
        Self {
            portal_spawn_time: 5.0,
        }
    }
}
