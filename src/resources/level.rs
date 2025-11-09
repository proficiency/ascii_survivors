use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum Level {
    #[default]
    Survival,
    Rest,
    Grassland,
    Dungeon,
}
