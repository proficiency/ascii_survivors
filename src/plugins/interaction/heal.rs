use crate::{
    effects::*,
    events::*,
    objects::{Campfire, Health, Message, PlayerTag},
    plugins::audio::*,
};
use bevy::prelude::*;

pub fn heal_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Health), With<PlayerTag>>,
    campfire_query: Query<(Entity, &Message), With<Campfire>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    if let Ok((player_entity, mut health)) = player_query.single_mut() {
        for (entity, _) in campfire_query.iter() {
            if health.current < health.max {
                health.current = health.max;

                commands.entity(entity).remove::<Message>();
                commands.entity(player_entity).insert(StatusEffect {
                    color: Color::linear_rgb(0.0, 1.0, 0.0),
                });

                audio_events.write(AudioEvent {
                    channel: AudioChannelType::Sfx,
                    command: AudioCommand::Play {
                        audio: "sfx/heal.ogg",
                        looped: false,
                        volume: Some(0.35),
                    },
                });
            }
        }
    }
}
