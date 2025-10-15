use bevy::prelude::*;

#[derive(Component)]
pub struct Message {
    pub text: String,
    pub timer: Timer,
}

impl Message {
    pub fn new(text: String, duration: f32) -> Self {
        Self {
            text,
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}
