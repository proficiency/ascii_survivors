use bevy::prelude::{Component, IVec2, Resource};

#[derive(Resource, Component)]
pub struct CameraOffset(pub IVec2);

impl Default for CameraOffset {
    fn default() -> Self {
        Self(IVec2::default())
    }
}
