use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct KillCount {
    pub enemies: u32,
}
