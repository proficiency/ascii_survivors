use bevy::prelude::*;

use crate::{
    plugins::core::GameSet,
    plugins::interaction::{apply_interaction_messages, heal_player_system, interaction_system},
    plugins::world::{
        portal_transition::portal_transition_system, shop_npc_spawn::spawn_shop_npcs_on_rest_level,
    },
    resources::GameState,
};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interaction_system,
                apply_interaction_messages,
                heal_player_system,
                portal_transition_system,
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(spawn_shop_npcs_on_rest_level),
        );
    }
}
