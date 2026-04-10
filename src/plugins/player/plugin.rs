use bevy::prelude::*;

use super::{level_up_system, player_movement};
use crate::objects::{
    auto_cast, orb_movement, process_collisions, process_orb_collection, process_projectiles,
};
use crate::plugins::core::GameSet;
use crate::plugins::enemy::{boss_ai, spawn_enemies};
use crate::plugins::world::{portal_transition_system, spawn_portal_after_survival, update_scene_lock};
use crate::resources::GameState;

pub struct PlayerMovementPlugin;
pub struct PlayerCombatPlugin;
pub struct PlayerPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(update_scene_lock)
                    .after(spawn_portal_after_survival)
                    .before(spawn_enemies),
                orb_movement
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(process_collisions),
                process_orb_collection
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(orb_movement),
            ),
        );
    }
}

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                auto_cast
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(boss_ai)
                    .after(portal_transition_system),
                process_projectiles
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(auto_cast),
            ),
        );
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerMovementPlugin, PlayerCombatPlugin))
            .add_systems(
                Update,
                level_up_system
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(process_orb_collection),
            );
    }
}
