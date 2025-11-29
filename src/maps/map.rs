use crate::maps::tile::{Tile, TileType};
use anyhow::{Result, anyhow};
use bevy::prelude::*;
use rexpaint::*;
use std::io::Cursor;
use std::path::Path;

const BASE_MAP_WIDTH: usize = 80;
const BASE_MAP_HEIGHT: usize = 50;
const GRASSLAND_WIDTH: usize = 140;
const GRASSLAND_HEIGHT: usize = 90;

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

    pub fn from_xp_data(xp_data: &[u8], name: String) -> Result<Self> {
        let mut cursor = Cursor::new(xp_data);
        let xp_file = XpFile::read(&mut cursor)?;

        if xp_file.layers.is_empty() {
            return Err(anyhow!("XP file has no layers"));
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
            '=' => TileType::Bridge,
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

pub fn load_map_system(mut commands: Commands, level: Res<crate::resources::level::Level>) {
    // For now, we'll create a simple test map
    // In the future, we'll load actual rexpaint files based on the level
    let map = create_test_map_for_level(*level);
    commands.insert_resource(map);
}

pub fn create_test_map_for_level(level: crate::resources::level::Level) -> Map {
    let mut map = match level {
        crate::resources::level::Level::Grassland => load_or_generate_grassland_map(),
        crate::resources::level::Level::Dungeon => generate_dungeon_map(),
        crate::resources::level::Level::Rest => generate_rest_area_map(),
        crate::resources::level::Level::Survival => generate_survival_map(),
    };

    mark_all_explored(&mut map);
    map
}

fn fill_map(map: &mut Map, tile_type: TileType) {
    for column in map.tiles.iter_mut() {
        for tile in column.iter_mut() {
            *tile = Tile::new(tile_type);
        }
    }
}

fn carve_meandering_river(
    map: &mut Map,
    base_y: f32,
    amplitude: f32,
    wavelength: f32,
    half_width: usize,
) {
    for x in 0..map.width {
        let center_y = river_center_y(x, base_y, amplitude, wavelength);

        for dy in -(half_width as i32)..=(half_width as i32) {
            let y = center_y + dy;
            if map.in_bounds(x as i32, y) {
                map.tiles[x][y as usize] = Tile::new(TileType::Water);
            }
        }
    }
}

fn place_hills(map: &mut Map) {
    let hill_centers = [
        IVec2::new((map.width as i32 * 2) / 10, (map.height as i32 * 3) / 10),
        IVec2::new((map.width as i32 * 7) / 10, (map.height as i32 * 4) / 10),
        IVec2::new((map.width as i32 * 5) / 10, (map.height as i32 * 7) / 10),
        IVec2::new((map.width as i32 * 8) / 10, (map.height as i32 * 7) / 10),
    ];
    let hill_radii = [7, 9, 6, 5];

    for (center, radius) in hill_centers.into_iter().zip(hill_radii.into_iter()) {
        carve_hill(map, center, radius);
    }
}

fn place_bridge_over_river(
    map: &mut Map,
    base_y: f32,
    amplitude: f32,
    wavelength: f32,
    half_width: usize,
    bridge_x: usize,
) {
    let center_y = river_center_y(bridge_x, base_y, amplitude, wavelength);
    for dx in -1..=1 {
        let x = bridge_x as i32 + dx;
        for dy in -(half_width as i32 + 1)..=(half_width as i32 + 1) {
            let y = center_y + dy;
            if map.in_bounds(x, y) {
                map.tiles[x as usize][y as usize] = Tile::new(TileType::Bridge);
            }
        }
    }
}

fn river_center_y(x: usize, base_y: f32, amplitude: f32, wavelength: f32) -> i32 {
    let x_wave = x as f32 / wavelength;
    let offset = x_wave.sin() * amplitude + (x_wave * 0.5).cos() * (amplitude * 0.4);
    (base_y + offset).round() as i32
}

fn carve_hill(map: &mut Map, center: IVec2, radius: i32) {
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = center.x + dx;
                let y = center.y + dy;
                if map.in_bounds(x, y) {
                    map.tiles[x as usize][y as usize] = Tile::new(TileType::Stone);
                }
            }
        }
    }
}

fn load_or_generate_grassland_map() -> Map {
    let name = "Grassland".to_string();
    let xp_path = Path::new("assets/maps/grassland.xp");

    if let Ok(bytes) = std::fs::read(xp_path) {
        if let Ok(map) = Map::from_xp_data(&bytes, name.clone()) {
            return map;
        }
    }

    generate_grassland_map(name)
}

fn generate_grassland_map(name: String) -> Map {
    let mut map = Map::new(GRASSLAND_WIDTH, GRASSLAND_HEIGHT, name);
    fill_map(&mut map, TileType::Grass);
    let river_base_y = map.height as f32 * 0.55;
    let river_amplitude = 7.5;
    let river_wavelength = 14.0;
    let river_half_width = 3;
    let bridge_x = map.width / 2;
    carve_meandering_river(
        &mut map,
        river_base_y,
        river_amplitude,
        river_wavelength,
        river_half_width,
    );
    place_bridge_over_river(
        &mut map,
        river_base_y,
        river_amplitude,
        river_wavelength,
        river_half_width,
        bridge_x,
    );
    place_hills(&mut map);
    map
}

fn generate_dungeon_map() -> Map {
    let mut map = Map::new(BASE_MAP_WIDTH, BASE_MAP_HEIGHT, "Dungeon".to_string());
    for x in 25..55 {
        map.tiles[x][25] = Tile::wall();
    }
    for y in 20..30 {
        map.tiles[30][y] = Tile::wall();
        map.tiles[50][y] = Tile::wall();
    }
    map
}

fn generate_rest_area_map() -> Map {
    let mut map = Map::new(BASE_MAP_WIDTH, BASE_MAP_HEIGHT, "Rest Area".to_string());
    map.tiles[40][25] = Tile::new(TileType::Empty);
    map
}

fn generate_survival_map() -> Map {
    let mut map = Map::new(BASE_MAP_WIDTH, BASE_MAP_HEIGHT, "Survival Mode".to_string());
    map.tiles[40][20] = Tile::new(TileType::Empty);
    map.tiles[41][20] = Tile::new(TileType::Wall);
    map.tiles[42][20] = Tile::new(TileType::Water);
    map.tiles[43][20] = Tile::new(TileType::Grass);
    map.tiles[44][20] = Tile::new(TileType::Stone);
    map.tiles[45][20] = Tile::new(TileType::Door);
    map
}

fn mark_all_explored(map: &mut Map) {
    for x in 0..map.width {
        for y in 0..map.height {
            map.tiles[x][y].explored = true;
        }
    }
}

pub fn map_to_xp(map: &Map) -> XpFile {
    let mut xp = XpFile::new(map.width, map.height);
    if let Some(layer) = xp.layers.get_mut(0) {
        for x in 0..map.width {
            for y in 0..map.height {
                if let Some(tile) = map.get_tile(x as i32, y as i32) {
                    if let Some(cell) = layer.get_mut(x, y) {
                        cell.ch = tile.tile_type.to_char() as u32;
                        cell.fg = to_xp_color(tile.tile_type.to_color());
                        cell.bg = to_xp_color(tile.tile_type.to_bg_color());
                    }
                }
            }
        }
    }
    xp
}

fn to_xp_color(color: Color) -> XpColor {
    let srgb = color.to_srgba();
    let clamp_to_u8 = |value: f32| (value.clamp(0.0, 1.0) * 255.0) as u8;
    XpColor::new(
        clamp_to_u8(srgb.red),
        clamp_to_u8(srgb.green),
        clamp_to_u8(srgb.blue),
    )
}
