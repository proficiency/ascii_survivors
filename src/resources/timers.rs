use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.25, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct ProjectileCooldownTimer(pub Timer);

impl Default for ProjectileCooldownTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct PlayerMovementTimer(pub Timer);

impl Default for PlayerMovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct EnemyMovementTimer(pub Timer);

impl Default for EnemyMovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.35, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct DamageEffectTimer(pub Timer);

impl Default for DamageEffectTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct LoadingTimer(pub Timer);

impl Default for LoadingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct FadeTimer(pub Timer);

impl Default for FadeTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct SurvivalTimer(pub Timer);

impl Default for SurvivalTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3600.0, TimerMode::Once))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortalTransitionState {
    Inactive,
    BuildingUp,
    BreakingDown,
}

#[derive(Resource)]
pub struct PortalTransition {
    pub timer: Timer,
    pub state: PortalTransitionState,
    pub progress: f32, // 0.0 to 1.0
}

impl Default for PortalTransition {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            state: PortalTransitionState::Inactive,
            progress: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct LevelTransitionTimer(pub Timer);

impl Default for LevelTransitionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct InteractionTimer(pub Timer);

impl Default for InteractionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}
