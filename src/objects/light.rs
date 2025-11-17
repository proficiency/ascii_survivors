use bevy::prelude::*;

/// Marks an entity as contributing to the dynamic lighting overlay.
#[derive(Component, Clone, Copy)]
pub struct LightEmitter {
    /// Radius of the light in world units (before pixel upscaling).
    pub radius: f32,
    /// Overall intensity multiplier applied after falloff.
    pub intensity: f32,
    /// Exponent used while computing falloff (higher values = steeper falloff).
    pub falloff: f32,
    /// Base color (in linear space) contributed by this light.
    pub color: LinearRgba,
}

impl LightEmitter {
    pub fn new(color: Color, radius: f32, intensity: f32, falloff: f32) -> Self {
        Self {
            radius,
            intensity,
            falloff,
            color: LinearRgba::from(color),
        }
    }

    /// Soft glow used while the player explores survival arenas.
    pub fn player_default() -> Self {
        Self::new(
            Color::linear_rgba(0.35, 0.55, 1.0, 0.45),
            6.5,
            0.8,
            2.0,
        )
    }

    /// Warm and wide light emitted by rest area campfires.
    pub fn campfire() -> Self {
        Self::new(
            Color::linear_rgba(1.0, 0.62, 0.2, 0.4),
            13.0,
            0.85,
            1.65,
        )
    }
}

/// Adds a natural flicker to a [`LightEmitter`].
#[derive(Component)]
pub struct LightFlicker {
    pub intensity_range: (f32, f32),
    pub radius_range: (f32, f32),
    pub lerp_speed: f32,
    pub timer: Timer,
    pub target_intensity: f32,
    pub target_radius: f32,
}

impl LightFlicker {
    pub fn new(
        intensity_range: (f32, f32),
        radius_range: (f32, f32),
        change_rate: f32,
        lerp_speed: f32,
        initial_intensity: f32,
        initial_radius: f32,
    ) -> Self {
        Self {
            intensity_range,
            radius_range,
            lerp_speed,
            timer: Timer::from_seconds(change_rate, TimerMode::Repeating),
            target_intensity: initial_intensity,
            target_radius: initial_radius,
        }
    }

    pub fn campfire() -> Self {
        Self::new(
            (0.55, 0.95),
            (11.0, 14.0),
            0.17,
            6.0,
            0.85,
            13.0,
        )
    }
}
