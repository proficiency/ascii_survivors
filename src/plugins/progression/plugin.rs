use bevy::prelude::*;

use crate::{
    objects::{Health, PlayerTag},
    plugins::core::GameSet,
    plugins::world::{
        despawn_portals, level_transition_system,
        portal_spawn::spawn_portal_after_survival, setup_level_transition,
    },
    resources::{GameState, SurvivalTimer},
};

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LevelTransition),
            (setup_level_transition, despawn_portals).chain(),
        )
        .add_systems(
            Update,
            (
                update_survival_timer
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(spawn_portal_after_survival),
                death_detection_system
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(crate::effects::update_status_effect),
                level_transition_system
                    .run_if(in_state(GameState::LevelTransition))
                    .in_set(GameSet::Gameplay),
            ),
        );
    }
}

fn death_detection_system(
    player_query: Query<&Health, With<PlayerTag>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_events: EventWriter<crate::events::AudioEvent>,
) {
    if let Ok(player) = player_query.single()
        && player.current <= 0.0
    {
        next_state.set(GameState::GameOver);
        audio_events.write(crate::events::AudioEvent {
            channel: crate::plugins::audio::AudioChannelType::Music,
            command: crate::plugins::audio::AudioCommand::Stop,
        });
    }
}

fn update_survival_timer(time: Res<Time>, mut survival_timer: ResMut<SurvivalTimer>) {
    survival_timer.0.tick(time.delta());
}
