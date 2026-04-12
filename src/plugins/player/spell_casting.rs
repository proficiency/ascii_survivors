use crate::{
    events::*,
    objects::{Boss, Enemy, GridPosition, PlayerTag},
    plugins::audio::*,
    resources::*,
    spells::{Arcanum, SpellType},
    upgrades::PlayerModifiers,
};
use bevy::prelude::*;

pub fn mana_regen_system(
    mut player_query: Query<&mut Arcanum, With<PlayerTag>>,
    time: Res<Time>,
) {
    if let Ok(mut arcanum) = player_query.single_mut() {
        arcanum.regenerate_mana(time.delta_secs());
    }
}
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
    modifiers: Res<PlayerModifiers>,
) {
    let base_cooldown = 1.2_f32;
    let modified_cooldown = (base_cooldown * modifiers.cast_cooldown_multiplier).max(0.1);
    timer.0.set_duration(std::time::Duration::from_secs_f32(modified_cooldown));
    timer.0.tick(time.delta());

    if !timer.0.finished() || scene_lock.0 {
        return;
    }

    if let Ok((pos, mut arcanum)) = player_query.single_mut() {
        if !arcanum.can_cast_spell(SpellType::Fireball, &modifiers) {
            return;
        }

        let target_count = modifiers.get_total_projectiles(SpellType::Fireball) as usize;
        let targets = find_nearest_targets(pos.world, &enemy_query, &boss_query, target_count);

        if targets.is_empty() {
            return;
        }

        let player_pos = pos.world;
        arcanum
            .cast_spell(&mut commands, SpellType::Fireball, player_pos, &targets, &modifiers)
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

// collect up to `count` nearest targets, prioritizing bosses
pub fn find_nearest_targets(
    player_pos: IVec2,
    enemy_query: &Query<(Entity, &Enemy)>,
    boss_query: &Query<(Entity, &Boss)>,
    count: usize,
) -> Vec<Entity> {
    let mut candidates: Vec<(Entity, i32, bool)> = Vec::new();

    for (entity, boss) in boss_query.iter() {
        let dist = (boss.get_head_position() - player_pos).length_squared();
        candidates.push((entity, dist, true));
    }

    for (entity, enemy) in enemy_query.iter() {
        let dist = (enemy.position - player_pos).length_squared();
        candidates.push((entity, dist, false));
    }

    // bosses first, then by distance
    candidates.sort_by(|a, b| b.2.cmp(&a.2).then(a.1.cmp(&b.1)));

    candidates.iter().take(count).map(|(e, _, _)| *e).collect()
}
