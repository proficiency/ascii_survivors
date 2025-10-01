use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

use crate::objects::enemy::Enemy;
use crate::resources::camera::CameraOffset;
use crate::resources::timers::{EnemySpawnTimer, SurvivalTimer};
use crate::resources::ruleset::Ruleset;
use crate::resources::GameState;

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
    game_state: Res<State<GameState>>,
) {
    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time 
        || *game_state.get() == GameState::LevelTransition {
        return;
    }
    
    if let Ok(terminal) = terminal_query.single() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let size = terminal.size();
            let mut rng = rand::thread_rng();

            // choose a random edge to spawn the enemy at
            let (x, y) = match rng.gen_range(0..4) {
                // top edge
                0 => (rng.gen_range(0..size[0] as i32), 0),
                // bottom edge
                1 => (rng.gen_range(0..size[0] as i32), size[1] as i32 - 1),
                // left edge
                2 => (0, rng.gen_range(0..size[1] as i32)),
                // right edge
                _ => (size[0] as i32 - 1, rng.gen_range(0..size[1] as i32)),
            };

            // spawn the enemy offscreen
            let spawn_position = IVec2::new(x, y) - camera_offset.0;
            commands.spawn((Enemy::new(spawn_position),));
        }
    }
}
