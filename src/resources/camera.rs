use bevy::prelude::{Component, IVec2, Resource};

#[derive(Resource, Component)]
pub struct CameraOffset(pub IVec2);
