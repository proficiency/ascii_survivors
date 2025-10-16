use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Resource)]
pub enum Level {
    Grassland,
    Dungeon,
    Rest,
    Survival,
}

impl Default for Level {
    fn default() -> Self {
        Level::Grassland
    }
}

impl std::fmt::Debug for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Grassland => write!(f, "Grassland"),
            Level::Dungeon => write!(f, "Dungeon"),
            Level::Rest => write!(f, "Rest"),
            Level::Survival => write!(f, "Survival"),
        }
    }
}

impl Level {
    // returns the filename for the REXPaint file for this level
    pub fn map_filename(&self) -> &'static str {
        match self {
            Level::Grassland => "grassland.xp",
            Level::Dungeon => "dungeon.xp",
            Level::Rest => "rest.xp",
            Level::Survival => "survival.xp",
        }
    }
}
