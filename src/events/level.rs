use crate::resources::Level;
use bevy::prelude::*;

#[derive(Clone, Copy, Event)]
pub struct LevelChangedEvent {
    pub new_level: Level,
}

#[derive(Clone, Copy, Event)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}
