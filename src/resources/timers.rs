use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct ProjectileCooldownTimer(pub Timer);

#[derive(Resource)]
pub struct PlayerMovementTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyMovementTimer(pub Timer);

#[derive(Resource)]
pub struct DamageEffectTimer(pub Timer);

#[derive(Resource)]
pub struct LoadingTimer(pub Timer);

#[derive(Resource)]
pub struct FadeTimer(pub Timer);

#[derive(Resource)]
pub struct SurvivalTimer(pub Timer);

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
