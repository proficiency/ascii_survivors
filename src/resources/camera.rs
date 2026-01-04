use bevy::prelude::{Component, IVec2, Resource};

#[derive(Resource, Component, Default)]
pub struct CameraOffset(pub IVec2);
