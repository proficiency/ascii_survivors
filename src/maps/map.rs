use crate::maps::tile::{Tile, TileType};
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use std::io::Cursor;
use rexpaint::*;

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub name: String,
}

impl Map {
    pub fn new(width: usize, height: usize, name: String) -> Self {
        let tiles = vec![vec![Tile::empty(); height]; width];
        Self {
            width,
            height,
            tiles,
            name,
        }
    }

    pub fn from_xp_data(xp_data: &[u8], name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(xp_data);
        let xp_file = XpFile::read(&mut cursor)?;

        if xp_file.layers.is_empty() {
            return Err("XP file has no layers".into());
        }

        // Use the first layer for our map
        let layer = &xp_file.layers[0];
        let width = layer.width;
        let height = layer.height;

        let mut map = Map::new(width, height, name);

        // convert XP cells to our tile format
        for x in 0..width {
            for y in 0..height {
                if let Some(cell) = layer.get(x, y) {
                    let tile_type = Self::char_to_tile_type(cell.ch as u8 as char);
                    map.tiles[x][y] = Tile::new(tile_type);
                }
            }
        }

        Ok(map)
    }

    fn char_to_tile_type(ch: char) -> TileType {
        match ch {
            '#' => TileType::Wall,
            '.' => TileType::Grass,
            '~' => TileType::Water,
            ':' => TileType::Stone,
            '+' => TileType::Door,
            _ => TileType::Empty,
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }

        self.tiles[x as usize][y as usize].tile_type.is_walkable()
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if !self.in_bounds(x, y) {
            return None;
        }

        Some(&self.tiles[x as usize][y as usize])
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if !self.in_bounds(x, y) {
            return None;
        }

        Some(&mut self.tiles[x as usize][y as usize])
    }
}

pub fn load_map_system(
    mut commands: Commands,
    level: Res<crate::resources::level::Level>,
) {
    // For now, we'll create a simple test map
    // In the future, we'll load actual rexpaint files based on the level
    let map = create_test_map_for_level(*level);
    commands.insert_resource(map);
}

fn create_test_map_for_level(level: crate::resources::level::Level) -> Map {
    let map_name = match level {
        crate::resources::level::Level::Grassland => "Grassland",
        crate::resources::level::Level::Dungeon => "Dungeon",
        crate::resources::level::Level::Rest => "Rest Area",
        crate::resources::level::Level::Survival => "Survival Mode",
    };
    
    let mut map = Map::new(80, 50, map_name.to_string());

    // enclose the map in walls, testing purposes
    for x in 0..80 {
        map.tiles[x][0] = Tile::wall();
        map.tiles[x][49] = Tile::wall();
    }
    
    for y in 0..50 {
        map.tiles[0][y] = Tile::wall();
        map.tiles[79][y] = Tile::wall();
    }
    
    match level {
        crate::resources::level::Level::Grassland => {
            for x in 10..70 {
                for y in 10..40 {
                    if x % 3 == 0 && y % 3 == 0 {
                        map.tiles[x][y] = Tile::new(TileType::Grass);
                    }
                }
            }
        },
        crate::resources::level::Level::Dungeon => {
            for x in 25..55 {
                map.tiles[x][25] = Tile::wall();
            }
            for y in 20..30 {
                map.tiles[30][y] = Tile::wall();
                map.tiles[50][y] = Tile::wall();
            }
        },
        crate::resources::level::Level::Rest => {
            // campfire in the center
            map.tiles[40][25] = Tile::new(TileType::Empty);
        },
        crate::resources::level::Level::Survival => {
            for x in 5..75 {
                for y in 5..45 {
                    if (x + y) % 5 == 0 {
                        map.tiles[x][y] = Tile::new(TileType::Stone);
                    }
                }
            }
        },
    }
    
    // mark all tiles as explored for testing
    for x in 0..map.width {
        for y in 0..map.height {
            map.tiles[x][y].explored = true;
        }
    }
    
    map
}
