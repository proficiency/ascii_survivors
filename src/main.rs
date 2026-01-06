mod debug;
mod effects;
mod events;
mod maps;
mod objects;
mod plugins;
mod resources;
mod scenes;
mod spells;
mod systems;

use crate::{
    debug::DebugPlugins,
    effects::*,
    objects::*,
    plugins::AsciiSurvivorsPlugins,
    resources::*,
    scenes::*,
    systems::*,
};

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Gameplay,
    Rendering,
    Cleanup,
}

fn main() {
    App::new()
        .add_plugins((
            AsciiSurvivorsPlugins,
            GameScenesPlugin,
            #[cfg(debug_assertions)]
            DebugPlugins,
        ))
        .configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Gameplay.after(GameSet::Input),
                GameSet::Rendering.after(GameSet::Gameplay),
                GameSet::Cleanup.after(GameSet::Rendering),
            ),
        )
        .add_systems(
            Update,
            (
                update_scene_lock
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(spawn_portal_after_survival)
                    .before(player_movement),
                (
                    spawn_portal_after_survival,
                    player_movement,
                    spawn_enemies,
                    spawn_bosses,
                    spawn_shop_npcs_on_rest_level,
                    interaction_system,
                    apply_interaction_messages,
                    heal_player_system,
                    portal_transition_system,
                    (
                        enemy_ai,
                        boss_ai,
                        auto_cast,
                        process_projectiles,
                        process_collisions,
                        orb_movement,
                        process_orb_collection,
                        campfire_animation_system,
                        ember_animation_system,
                        light_flicker_system,
                    )
                        .chain(),
                    update_status_effect,
                    systems::render::render_system,
                    render_message_system,
                    render_portal_transition,
                    despawn_entities,
                )
                    .chain()
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay),
            ),
        )
        .add_systems(
            Update,
            update_lighting_overlay
                .after(render_system)
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Rendering),
        )
        .run();
}
