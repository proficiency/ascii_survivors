use bevy::prelude::{Resource, Component, IVec2};

#[derive(Resource, Component)]
pub struct CameraOffset(pub IVec2);
