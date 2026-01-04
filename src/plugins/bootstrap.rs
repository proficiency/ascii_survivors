use crate::resources::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_ascii_terminal::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ASCII Survivors".into(),
                    visible: true,
                    present_mode: PresentMode::Fifo,
                    resolution: (640., 400.).into(),
                    ..default()
                }),
                ..default()
            }),
            TerminalPlugins,
        ))
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
        .init_resource::<KillCount>()
        .add_systems(
            Startup,
            (spawn_terminal, crate::systems::setup_lighting_overlay).chain(),
        );
    }
}

fn spawn_terminal(mut commands: Commands) {
    commands.spawn(Terminal::new([80, 50]));
    commands.spawn(TerminalCamera::new());
}
