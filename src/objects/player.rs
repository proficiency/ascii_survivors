use crate::spells::Arcanum;
use bevy::prelude::*;

pub fn experience_for_level(level: u32) -> u32 {
    (100.0 * (level as f32).powf(1.5)) as u32
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct PlayerTag;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GridPosition {
    pub tile: IVec2,
    pub world: IVec2,
}

impl GridPosition {
    pub fn new(tile: IVec2) -> Self {
        Self { tile, world: tile }
    }

    pub fn with_screen(screen: IVec2, world: IVec2) -> Self {
        Self { tile: screen, world }
    }
}

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct MoveSpeed(pub f32);

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(17.0)
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Experience {
    pub current: u32,
    pub level: u32,
    pub to_next: u32,
}

impl Experience {
    pub fn new(level: u32) -> Self {
        Self {
            current: 0,
            level,
            to_next: experience_for_level(level),
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub tag: PlayerTag,
    pub health: Health,
    pub grid_position: GridPosition,
    pub move_speed: MoveSpeed,
    pub experience: Experience,
    pub arcanum: Arcanum,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn at(tile: IVec2) -> Self {
        let mut bundle = Self {
            grid_position: GridPosition::new(tile),
            health: Health::new(100.0),
            move_speed: MoveSpeed::default(),
            experience: Experience::new(1),
            arcanum: Arcanum::new(),
            ..Default::default()
        };

        bundle.transform.translation = Vec3::new(tile.x as f32, tile.y as f32, 0.0);
        bundle
    }

    pub fn at_screen(screen: IVec2, world: IVec2) -> Self {
        let mut bundle = Self {
            grid_position: GridPosition::with_screen(screen, world),
            health: Health::new(100.0),
            move_speed: MoveSpeed::default(),
            experience: Experience::new(1),
            arcanum: Arcanum::new(),
            ..Default::default()
        };

        bundle.transform.translation = Vec3::new(world.x as f32, world.y as f32, 0.0);
        bundle
    }
}
