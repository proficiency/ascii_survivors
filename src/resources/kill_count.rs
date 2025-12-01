use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct KillCount {
    pub enemies: u32,
}
