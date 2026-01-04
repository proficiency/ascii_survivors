use bevy::prelude::Resource;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum Level {
    Survival,
    Rest,
    #[default]
    Grassland,
    Dungeon,
}
