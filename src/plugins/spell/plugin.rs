use crate::effects::update_status_effect;
use crate::objects::{process_fireballs, process_projectiles};
use crate::plugins::core::GameSet;
use crate::plugins::player::{spell_casting::SpellInputTimer, spell_casting::spell_casting_system};
use crate::plugins::rendering::render_system;
use crate::resources::GameState;
use bevy::prelude::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellInputTimer>()
            .add_systems(
                Update,
                process_fireballs
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(process_projectiles)
                    .before(update_status_effect),
            )
            .add_systems(
                Update,
                spell_casting_system
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(update_status_effect)
                    .before(render_system),
            );
    }
}
