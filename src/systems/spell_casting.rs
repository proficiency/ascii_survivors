use crate::objects::boss::Boss;
use crate::objects::enemy::Enemy;
use crate::objects::player::Player;
use crate::resources::{channels::Sfx, scene_lock::SceneLock};
use crate::spells::arcanum::SpellType;
use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;
use bevy_kira_audio::prelude::*;
#[derive(Resource)]
pub struct SpellInputTimer(pub Timer);

impl Default for SpellInputTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.2, TimerMode::Repeating))
    }
}

pub fn spell_casting_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    enemy_query: Query<(Entity, &Enemy)>,
    boss_query: Query<(Entity, &Boss)>,
    time: Res<Time>,
    mut timer: ResMut<SpellInputTimer>,
    scene_lock: Res<SceneLock>,
    audio: Res<AudioChannel<Sfx>>,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() || scene_lock.0 {
        return;
    }

    if let Ok(mut player) = player_query.single_mut() {
        player.arcanum.regenerate_mana(time.delta_secs());

        let mut nearest_target_entity: Option<Entity> = None;
        let mut min_distance = i32::MAX;

        // prioritize bosses
        for (boss_entity, boss) in boss_query.iter() {
            let boss_world_pos = boss.get_head_position();
            let player_world_pos = player.world_position;

            let distance = (boss_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(boss_entity);
            }
        }

        for (enemy_entity, enemy) in enemy_query.iter() {
            let enemy_world_pos = enemy.position;
            let player_world_pos = player.world_position;

            let distance = (enemy_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(enemy_entity);
            }
        }

        if let Some(target_entity) = nearest_target_entity {
            if player
                .arcanum
                .cast_spell(
                    &mut commands,
                    SpellType::Fireball,
                    player.world_position,
                    Some(target_entity),
                )
                .is_ok()
            {
                audio
                    .play(asset_server.load("sfx/25_Wind_01.wav"))
                    .with_volume(0.25);
            }
        }
    }
}

pub fn spell_render_system(mut query: Query<&mut Terminal>, player_query: Query<&Player>) {
    if let Ok(mut terminal) = query.single_mut() {
        if let Ok(player) = player_query.single() {
            let mana_ratio = player.arcanum.mana / player.arcanum.max_mana;
            let bar_height = 20;
            let filled_height = (bar_height as f32 * mana_ratio) as usize;
            for i in 0..bar_height {
                if i < filled_height {
                    terminal.put_char([0, 20 + i], '█');
                } else {
                    terminal.put_char([0, 20 + i], '░');
                }
            }
        }
    }
}
