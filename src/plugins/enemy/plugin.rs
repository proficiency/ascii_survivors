use bevy::prelude::*;

use crate::{
    objects::{process_collisions, process_projectiles},
    plugins::core::GameSet,
    plugins::enemy::{boss_ai, enemy_ai},
    plugins::enemy::{spawn_bosses, spawn_enemies},
    plugins::world::portal_transition_system,
    resources::GameState,
};

pub struct EnemyMovementPlugin;
pub struct EnemyCombatPlugin;
pub struct EnemyPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_ai
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(portal_transition_system)
                    .after(spawn_enemies)
                    .after(spawn_bosses),
                boss_ai
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(enemy_ai)
                    .after(spawn_bosses),
            ),
        );
    }
}

impl Plugin for EnemyCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_collisions
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(process_projectiles),
        );
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyMovementPlugin, EnemyCombatPlugin));
    }
}
