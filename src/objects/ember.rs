use bevy::prelude::*;

#[derive(Component)]
pub struct Ember {
    pub position: IVec2,
    pub velocity: IVec2,
    pub gravity: IVec2,
    pub lifetime: Timer,
}

impl Ember {
    pub fn new(position: IVec2, velocity: IVec2, lifetime: f32) -> Self {
        Self {
            position,
            velocity,
            gravity: IVec2::new(0, -1),
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
        }
    }
    
    pub fn update(&mut self, time: &Time) {
        self.lifetime.tick(time.delta());
        self.velocity += self.gravity;
        self.position += self.velocity;
    }
}
