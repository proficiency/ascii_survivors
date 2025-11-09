use crate::{effects::*, objects::*, resources::*};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn heal_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    campfire_query: Query<(Entity, &Message), With<Campfire>>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<Sfx>>,
) {
    if let Ok((player_entity, mut player)) = player_query.single_mut() {
        for (entity, _) in campfire_query.iter() {
            if player.health < player.max_health {
                player.health = player.max_health;

                audio
                    .play(asset_server.load("sfx/heal.ogg"))
                    .with_volume(0.35);

                commands.entity(entity).remove::<Message>();
                commands.entity(player_entity).insert(StatusEffect {
                    color: Color::linear_rgb(0.0, 1.0, 0.0),
                });
            }
        }
    }
}
