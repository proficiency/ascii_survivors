use crate::{effects::*, objects::*, plugins::audio::*};
use bevy::prelude::*;

pub fn heal_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    campfire_query: Query<(Entity, &Message), With<Campfire>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    if let Ok((player_entity, mut player)) = player_query.single_mut() {
        for (entity, _) in campfire_query.iter() {
            if player.health < player.max_health {
                player.health = player.max_health;

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
