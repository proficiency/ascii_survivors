use crate::resources::*;
use crate::systems::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_ascii_terminal::*;
use bevy_kira_audio::prelude::*;

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
            AudioPlugin,
        ))
        .init_state::<GameState>()
        .add_audio_channel::<Music>()
        .add_audio_channel::<Sfx>()
        .add_systems(
            Startup,
            (
                add_resources,
                spawn_terminal,
                list_gamepads,
                crate::systems::setup_lighting_overlay,
            )
                .chain(),
        );
    }
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    info!("Looking for gamepads...");
    for name in &gamepads {
        info!("Found gamepad: {}", name.0);
    }
}

fn add_resources(mut commands: Commands) {
    commands.insert_resource(SpellInputTimer::default());
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        1.25,
        TimerMode::Repeating,
    )));
    commands.insert_resource(ProjectileCooldownTimer(Timer::from_seconds(
        2.0,
        TimerMode::Once,
    )));
    commands.insert_resource(PlayerMovementTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
    commands.insert_resource(EnemyMovementTimer(Timer::from_seconds(
        0.35,
        TimerMode::Repeating,
    )));
    commands.insert_resource(DamageEffectTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(LoadingTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    commands.insert_resource(FadeTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    commands.insert_resource(SurvivalTimer(Timer::from_seconds(3600.0, TimerMode::Once)));
    commands.insert_resource(LevelTransitionTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
    commands.insert_resource(InteractionTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(PortalTransition::default());
    commands.insert_resource(CameraOffset(IVec2::default()));
    commands.insert_resource(SceneLock::default());
    commands.insert_resource(Ruleset::default());
    commands.insert_resource(Level::default());
    commands.insert_resource(KillCount::default());
}

fn spawn_terminal(mut commands: Commands) {
    commands.spawn(Terminal::new([80, 50]));
    commands.spawn(TerminalCamera::new());
}
