use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

use crate::maps::Map;
use crate::objects::boss::{Boss, BossType};
use crate::resources::*;

pub fn spawn_bosses(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
    game_state: Res<State<GameState>>,
    map: Option<Res<Map>>,
) {
    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time
        || *game_state.get() == GameState::LevelTransition
    {
        return;
    }

    if let Ok(terminal) = terminal_query.single() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let size = terminal.size();
            let mut rng = rand::rng();

            // todo: figure out when a boss should spawn
            if rng.random_bool(0.25) {
                let (x, y) = match rng.random_range(0..4) {
                    // top edge
                    0 => (rng.random_range(0..size[0] as i32), 0),
                    // bottom edge
                    1 => (rng.random_range(0..size[0] as i32), size[1] as i32 - 1),
                    // left edge
                    2 => (0, rng.random_range(0..size[1] as i32)),
                    // right edge
                    _ => (size[0] as i32 - 1, rng.random_range(0..size[1] as i32)),
                };

                let boss_type = match rng.random_range(0..2) {
                    0 => BossType::Snake,
                    _ => BossType::Giant,
                };

                let spawn_position = if let Some(map) = &map {
                    let (margin_x, margin_y) = match boss_type {
                        BossType::Snake => (12, 0), // snake extends 11 tiles left
                        BossType::Giant => (1, 1),
                    };

                    let max_x = map.width as i32 - 1 - margin_x;
                    let max_y = map.height as i32 - 1 - margin_y;
                    let min_x = margin_x;
                    let min_y = margin_y;

                    if max_x < min_x || max_y < min_y {
                        // fallback if map is too small
                        IVec2::new(x, y) - camera_offset.0
                    } else {
                        let sx = rng.random_range(min_x..=max_x);
                        let sy = rng.random_range(min_y..=max_y);
                        IVec2::new(sx, sy)
                    }
                } else {
                    // fallback to screen-based spawn
                    IVec2::new(x, y) - camera_offset.0
                };

                commands.spawn((Boss::new(spawn_position, boss_type),));
            }
        }
    }
}
