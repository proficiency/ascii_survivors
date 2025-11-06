use crate::spells::Arcanum;
use bevy::prelude::*;

pub fn experience_for_level(level: u32) -> u32 {
    (100.0 * (level as f32).powf(1.5)) as u32
}

#[derive(Component, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub position: IVec2,
    pub world_position: IVec2,
    pub speed: f32,
    pub experience: u32,
    pub level: u32,
    pub experience_to_next_level: u32,
    pub arcanum: Arcanum,
}

impl Player {
    pub fn new(position: IVec2) -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            position,
            world_position: position,
            speed: 17.0,
            experience: 0,
            level: 1,
            experience_to_next_level: experience_for_level(1),
            arcanum: Arcanum::new(),
        }
    }
}
