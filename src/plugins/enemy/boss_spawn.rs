use bevy::prelude::*;
use rand::RngExt;

use crate::{objects::*, resources::*};

#[allow(clippy::too_many_arguments)]
pub fn spawn_bosses(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    grid: Res<AsciiGrid>,
    camera_offset: Res<CameraOffset>,
    game_state: Res<State<GameState>>,
) {
    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time
        || *game_state.get() == GameState::LevelTransition
    {
        return;
    }

    timer.0.tick(time.delta());
    if timer.0.finished() {
        let size = grid.grid_size;
        let mut rng = rand::rng();

        // todo: figure out when a boss should spawn
        if rng.random_bool(0.25) {
            let (x, y) = match rng.random_range(0..4) {
                // top edge
                0 => (rng.random_range(0..size.x as i32), 0),
                // bottom edge
                1 => (rng.random_range(0..size.x as i32), size.y as i32 - 1),
                // left edge
                2 => (0, rng.random_range(0..size.y as i32)),
                // right edge
                _ => (size.x as i32 - 1, rng.random_range(0..size.y as i32)),
            };

            let boss_type = match rng.random_range(0..2) {
                0 => BossType::Snake,
                _ => BossType::Giant,
            };

            // ensure the boss spawns offscreen
            let spawn_position = IVec2::new(x, y) + camera_offset.0;
            commands.spawn((Boss::new(spawn_position, boss_type),));
        }
    }
}
