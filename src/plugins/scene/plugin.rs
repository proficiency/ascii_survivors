use crate::{
    events::{AudioEvent, UiActionEvent},
    objects::*,
    plugins::audio::{AudioChannelType, AudioCommand},
    plugins::core::GameSet,
    resources::*,
};
use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::FadingIn),
            (reset_fade_timer, play_start_sound).chain(),
        )
        .add_systems(
            Update,
            (
                loading_update_system
                    .run_if(in_state(GameState::Loading))
                    .in_set(GameSet::Gameplay),
                menu_input_system
                    .run_if(in_state(GameState::Menu))
                    .in_set(GameSet::Input),
                fade_in_update_system
                    .run_if(in_state(GameState::FadingIn))
                    .in_set(GameSet::Gameplay),
                (game_over_input_system, despawn_all_entities)
                    .chain()
                    .run_if(in_state(GameState::GameOver))
                    .in_set(GameSet::Cleanup),
            ),
        );
    }
}

#[allow(dead_code)]
fn menu_input_system(
    mut ui_events: EventReader<UiActionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in ui_events.read() {
        match event {
            UiActionEvent::Submit => next_state.set(GameState::FadingIn),
            UiActionEvent::Cancel => { /* maybe quit to title */ }
            UiActionEvent::Navigate(_dir) => { /* move focus by dir */ }
            UiActionEvent::Info => {}
        }
    }
}

fn loading_update_system(
    time: Res<Time>,
    mut loading_timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    loading_timer.0.tick(time.delta());

    if loading_timer.0.finished() {
        next_state.set(GameState::Menu);
    }
}

fn reset_fade_timer(mut fade_timer: ResMut<FadeTimer>) {
    fade_timer.0.reset();
}

fn play_start_sound(mut audio_events: EventWriter<AudioEvent>) {
    audio_events.write(AudioEvent {
        channel: AudioChannelType::Sfx,
        command: AudioCommand::Play {
            audio: "sfx/start.wav",
            looped: false,
            volume: Some(0.5),
        },
    });
}

fn fade_in_update_system(
    time: Res<Time>,
    mut fade_timer: ResMut<FadeTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    fade_timer.0.tick(time.delta());

    if fade_timer.0.finished() {
        next_state.set(GameState::Game);
    }
}

fn despawn_all_entities(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerTag>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    orb_query: Query<Entity, With<Orb>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in orb_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn game_over_input_system(
    mut ui_events: EventReader<UiActionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut camera_offset: ResMut<CameraOffset>,
) {
    for event in ui_events.read() {
        match event {
            UiActionEvent::Submit => {
                // restart
                camera_offset.0 = IVec2::default();
                next_state.set(GameState::Game);
            }
            UiActionEvent::Cancel => {
                // back to menu
                camera_offset.0 = IVec2::default();
                next_state.set(GameState::Menu);
            }
            _ => {}
        }
    }
}
