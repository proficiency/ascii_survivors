use crate::Level;
use bevy::prelude::*;

#[derive(Clone, Copy, Event)]
pub struct LevelChangedEvent {
    pub new_level: Level,
}
