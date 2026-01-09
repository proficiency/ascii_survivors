use crate::{events::LevelUpEvent, objects::Player};
use bevy::prelude::*;

const HEALTH_PER_LEVEL: f32 = 10.0;
const MANA_PER_LEVEL: f32 = 5.0;
const SPEED_PER_LEVEL: f32 = 0.5;

pub fn level_up_system(
    mut level_up_events: EventReader<LevelUpEvent>,
    mut player_query: Query<&mut Player>,
) {
    if level_up_events.is_empty() {
        return;
    }

    for event in level_up_events.read() {
        let Ok(mut player) = player_query.get_mut(event.entity) else {
            continue;
        };

        player.max_health += HEALTH_PER_LEVEL;
        player.health = player.max_health;
        player.arcanum.max_mana += MANA_PER_LEVEL;
        player.arcanum.mana = player.arcanum.max_mana;
        player.speed += SPEED_PER_LEVEL;
    }
}
