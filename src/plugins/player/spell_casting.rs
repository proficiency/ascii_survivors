use crate::{
    events::*,
    objects::{Boss, Enemy, GridPosition, PlayerTag},
    plugins::audio::*,
    resources::*,
    spells::{Arcanum, SpellType},
};
use bevy::prelude::*;
#[derive(Resource)]
pub struct SpellInputTimer(pub Timer);

impl Default for SpellInputTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.2, TimerMode::Repeating))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spell_casting_system(
    mut commands: Commands,
    mut player_query: Query<(&GridPosition, &mut Arcanum), With<PlayerTag>>,
    enemy_query: Query<(Entity, &Enemy)>,
    boss_query: Query<(Entity, &Boss)>,
    time: Res<Time>,
    mut timer: ResMut<SpellInputTimer>,
    scene_lock: Res<SceneLock>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() || scene_lock.0 {
        return;
    }

    if let Ok((pos, mut arcanum)) = player_query.single_mut() {
        arcanum.regenerate_mana(time.delta_secs());
        if !arcanum.can_cast_spell(SpellType::Fireball) {
            return;
        }

        let mut nearest_target_entity: Option<Entity> = None;
        let mut min_distance = i32::MAX;

        // prioritize bosses
        for (boss_entity, boss) in boss_query.iter() {
            let boss_world_pos = boss.get_head_position();
            let player_world_pos = pos.world;

            let distance = (boss_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(boss_entity);
            }
        }

        for (enemy_entity, enemy) in enemy_query.iter() {
            let enemy_world_pos = enemy.position;
            let player_world_pos = pos.world;

            let distance = (enemy_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(enemy_entity);
            }
        }

        if let Some(target_entity) = nearest_target_entity {
            let player_pos = pos.world;
            arcanum
                .cast_spell(
                    &mut commands,
                    SpellType::Fireball,
                    player_pos,
                    Some(target_entity),
                )
                .ok();

            audio_events.write(AudioEvent {
                channel: AudioChannelType::Sfx,
                command: AudioCommand::Play {
                    audio: "sfx/13_Ice_explosion_01.wav",
                    looped: false,
                    volume: Some(0.25),
                },
            });
        }
    }
}
