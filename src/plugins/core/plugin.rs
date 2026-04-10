use bevy::prelude::*;

use crate::{
    events::{LevelChangedEvent, LevelUpEvent},
    plugins::interaction::InteractionMessageEvent,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Gameplay,
    Rendering,
    Cleanup,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionMessageEvent>()
            .add_event::<LevelChangedEvent>()
            .add_event::<LevelUpEvent>()
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Gameplay.after(GameSet::Input),
                    GameSet::Rendering.after(GameSet::Gameplay),
                    GameSet::Cleanup.after(GameSet::Rendering),
                ),
            );
    }
}
