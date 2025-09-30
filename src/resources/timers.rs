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
