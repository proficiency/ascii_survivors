use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

use crate::maps::Map;
use crate::objects::enemy::Enemy;
use crate::resources::GameState;
use crate::resources::camera::CameraOffset;
use crate::resources::ruleset::Ruleset;
use crate::resources::timers::{EnemySpawnTimer, SurvivalTimer};

pub fn spawn_enemies(
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

            // choose a random edge to spawn the enemy at
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

            let desired_world = IVec2::new(x, y) - camera_offset.0;

            let spawn_position = if let Some(map) = &map {
                let max_x = (map.width as i32 - 1).max(0);
                let max_y = (map.height as i32 - 1).max(0);
                let mut attempts = 0;
                let mut rng = rand::rng();

                loop {
                    let clamped = IVec2::new(
                        desired_world.x.clamp(0, max_x),
                        desired_world.y.clamp(0, max_y),
                    );

                    if map.is_walkable(clamped.x, clamped.y) {
                        break clamped;
                    }

                    // fallback: random edge within map bounds
                    let (edge_x, edge_y) = match rng.random_range(0..4) {
                        0 => (rng.random_range(0..=max_x), 0),
                        1 => (rng.random_range(0..=max_x), max_y),
                        2 => (0, rng.random_range(0..=max_y)),
                        _ => (max_x, rng.random_range(0..=max_y)),
                    };
                    let candidate = IVec2::new(edge_x, edge_y);
                    if map.is_walkable(candidate.x, candidate.y) {
                        break candidate;
                    }

                    attempts += 1;
                    if attempts > 32 {
                        // give up spawning this tick if no spot found
                        return;
                    }
                }
            } else {
                desired_world
            };

            commands.spawn((Enemy::new(spawn_position),));
        }
    }
}
