use crate::effects::StatusEffect;
use crate::objects::campfire::Campfire;
use crate::objects::message::Message;
use crate::objects::player::Player;
use crate::resources::sound::SoundManager;
use bevy::prelude::*;
use std::path::PathBuf;

pub fn heal_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    campfire_query: Query<(Entity, &Message), With<Campfire>>,
    mut sound_manager: ResMut<SoundManager>,
) {
    if let Ok((player_entity, mut player)) = player_query.single_mut() {
        for (entity, _) in campfire_query.iter() {
            if player.health < player.max_health {
                player.health = player.max_health;
                sound_manager
                    .play_sound(PathBuf::from("./assets/sfx/45_Charge_05.wav"), -5.0)
                    .expect("Failed to play healing sound");
                commands.entity(entity).remove::<Message>();
                commands.entity(player_entity).insert(StatusEffect {
                    color: Color::linear_rgb(0.0, 1.0, 0.0),
                });
            }
        }
    }
}
