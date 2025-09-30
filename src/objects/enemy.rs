use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub position: IVec2,
    pub speed: f32,
    pub damage: f32,
}

impl Enemy {
    pub fn new(position: IVec2) -> Self {
        Self {
            health: 50.0, // two hit kill
            position,
            speed: 0.5,
            damage: 10.0,
        }
    }
}
