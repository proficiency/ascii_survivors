use bevy::prelude::*;

#[derive(Component)]
pub struct ShopNpc {
    pub position: IVec2,
}

impl ShopNpc {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
