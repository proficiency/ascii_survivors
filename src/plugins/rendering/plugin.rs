use bevy::prelude::*;

use crate::{
    effects::update_status_effect,
    plugins::core::GameSet,
    plugins::interaction::render_message_system,
    plugins::rendering::{render_system, update_lighting_overlay},
    plugins::world::render_portal_transition,
    resources::GameState,
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_system,
                render_message_system,
                render_portal_transition,
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(update_status_effect),
        )
        .add_systems(
            Update,
            update_lighting_overlay
                .after(render_system)
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Rendering),
        );
    }
}
