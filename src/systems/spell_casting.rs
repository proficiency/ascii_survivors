use bevy::prelude::*;

#[derive(Resource)]
pub struct SpellInputTimer(pub Timer);

impl Default for SpellInputTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}
