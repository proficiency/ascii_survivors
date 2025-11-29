use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Wall,
    Water,
    Grass,
    Stone,
    Door,
    Bridge,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Empty | TileType::Grass | TileType::Door | TileType::Bridge => true,
            TileType::Wall | TileType::Water | TileType::Stone => false,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Wall => '#',
            TileType::Water => '~',
            TileType::Grass => '.',
            TileType::Stone => ':',
            TileType::Door => '+',
            TileType::Bridge => '=',
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            TileType::Empty => Color::linear_rgb(0.0, 0.0, 0.0),
            TileType::Wall => Color::linear_rgb(0.5, 0.5, 0.5),
            TileType::Water => Color::linear_rgb(0.0, 0.0, 1.0),
            TileType::Grass => Color::linear_rgb(0.0, 0.5, 0.0),
            TileType::Stone => Color::linear_rgb(0.6, 0.6, 0.6),
            TileType::Door => Color::linear_rgb(0.5, 0.25, 0.0),
            TileType::Bridge => Color::linear_rgb(0.65, 0.4, 0.2),
        }
    }

    pub fn to_bg_color(&self) -> Color {
        match self {
            TileType::Grass => Color::linear_rgb(0.02, 0.12, 0.02),
            TileType::Water => Color::linear_rgb(0.0, 0.02, 0.1),
            TileType::Stone => Color::linear_rgb(0.08, 0.08, 0.08),
            TileType::Wall => Color::linear_rgb(0.1, 0.1, 0.1),
            TileType::Bridge => Color::linear_rgb(0.05, 0.03, 0.02),
            TileType::Door => Color::linear_rgb(0.05, 0.03, 0.02),
            TileType::Empty => Color::linear_rgb(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub explored: bool,
    pub visible: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            explored: false,
            visible: false,
        }
    }

    pub fn empty() -> Self {
        Self::new(TileType::Empty)
    }
    pub fn wall() -> Self {
        Self::new(TileType::Wall)
    }
}
