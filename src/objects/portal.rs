use bevy::prelude::*;

#[derive(Component)]
pub struct Portal {
    pub position: IVec2,
}

impl Portal {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
