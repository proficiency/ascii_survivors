use bevy::prelude::*;

use crate::{
    effects::update_status_effect,
    objects::process_orb_collection,
    plugins::ambient::{
        campfire_animation::campfire_animation_system, ember_animation::ember_animation_system,
        light_flicker::light_flicker_system,
    },
    plugins::core::GameSet,
    plugins::player::player_movement,
    plugins::world::{spawn_portal_after_survival, update_scene_lock},
    resources::GameState,
};

pub struct AmbientPlugin;

impl Plugin for AmbientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_scene_lock
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(spawn_portal_after_survival)
                    .before(player_movement),
                campfire_animation_system,
                ember_animation_system,
                light_flicker_system,
                update_status_effect.after(process_orb_collection),
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay),
        );
    }
}
