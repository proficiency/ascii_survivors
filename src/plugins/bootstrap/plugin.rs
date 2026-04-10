use crate::resources::*;
use bevy::{prelude::*, window::PresentMode};

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ASCII Survivors".into(),
                visible: true,
                present_mode: PresentMode::Fifo,
                resolution: (1600., 900.).into(),
                ..default()
            }),
            ..default()
        }),))
            .init_state::<GameState>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<ProjectileCooldownTimer>()
            .init_resource::<PlayerMovementTimer>()
            .init_resource::<EnemyMovementTimer>()
            .init_resource::<DamageEffectTimer>()
            .init_resource::<LoadingTimer>()
            .init_resource::<FadeTimer>()
            .init_resource::<SurvivalTimer>()
            .init_resource::<LevelTransitionTimer>()
            .init_resource::<InteractionTimer>()
            .init_resource::<PortalTransition>()
            .init_resource::<CameraOffset>()
            .init_resource::<SceneLock>()
            .init_resource::<Ruleset>()
            .init_resource::<Level>()
            .init_resource::<KillCount>();
    }
}
