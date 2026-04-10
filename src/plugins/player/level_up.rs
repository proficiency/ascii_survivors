use crate::{
    events::LevelUpEvent,
    objects::{Health, MoveSpeed, PlayerTag},
    spells::Arcanum,
};
use bevy::prelude::*;

const HEALTH_PER_LEVEL: f32 = 10.0;
const MANA_PER_LEVEL: f32 = 5.0;
const SPEED_PER_LEVEL: f32 = 0.5;

pub fn level_up_system(
    mut level_up_events: EventReader<LevelUpEvent>,
    mut player_query: Query<(&mut Health, &mut Arcanum, &mut MoveSpeed), With<PlayerTag>>,
) {
    if level_up_events.is_empty() {
        return;
    }

    for event in level_up_events.read() {
        let Ok((mut health, mut arcanum, mut speed)) = player_query.get_mut(event.entity) else {
            continue;
        };

        health.max += HEALTH_PER_LEVEL;
        health.current = health.max;
        arcanum.max_mana += MANA_PER_LEVEL;
        arcanum.mana = arcanum.max_mana;
        **speed += SPEED_PER_LEVEL;
    }
}
