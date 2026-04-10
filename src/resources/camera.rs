use bevy::prelude::{Component, IVec2, Resource, Vec2};

#[derive(Resource, Component, Default)]
pub struct CameraOffset(pub IVec2);

#[derive(Resource, Debug, Clone, Copy)]
pub struct CinematicCamera {
    pub current_offset: Vec2,
    pub target_offset: Vec2,
    pub smoothing: f32,
}

impl Default for CinematicCamera {
    fn default() -> Self {
        Self {
            current_offset: Vec2::ZERO,
            target_offset: Vec2::ZERO,
            smoothing: 12.0,
        }
    }
}
