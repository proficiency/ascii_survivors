use bevy::prelude::*;

use super::apply::apply_selected_upgrade;
use super::pool::{select_random_upgrades, UpgradePool};
use super::selection_input::{upgrade_selection_input, UpgradeNavTimer};
use super::selection_render::render_upgrade_selection;
use crate::objects::PlayerTag;
use crate::plugins::core::GameSet;
use crate::resources::GameState;
use crate::upgrades::*;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowUpgradeSelectionEvent>()
            .add_event::<UpgradeSelectedEvent>()
            .init_resource::<UpgradePool>()
            .init_resource::<PendingUpgradeSelections>()
            .init_resource::<UpgradeNavTimer>()
            .init_resource::<PlayerModifiers>()
            .add_systems(
                Update,
                handle_show_upgrade_selection
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay),
            )
            .add_systems(
                Update,
                upgrade_selection_input
                    .run_if(in_state(GameState::UpgradeSelection))
                    .in_set(GameSet::Input),
            )
            .add_systems(
                Update,
                render_upgrade_selection
                    .run_if(in_state(GameState::UpgradeSelection))
                    .in_set(GameSet::Rendering),
            )
            .add_systems(
                OnExit(GameState::UpgradeSelection),
                (apply_selected_upgrade, check_pending_upgrades).chain(),
            );
    }
}

fn handle_show_upgrade_selection(
    mut events: EventReader<ShowUpgradeSelectionEvent>,
    mut pending: ResMut<PendingUpgradeSelections>,
    mut next_state: ResMut<NextState<GameState>>,
    pool: Res<UpgradePool>,
    player_query: Query<&PlayerUpgrades, With<PlayerTag>>,
    selection: Option<Res<UpgradeSelectionState>>,
    mut commands: Commands,
) {
    for event in events.read() {
        pending.queue.push_back(event.source);
    }

    // don't start a new selection if one is already showing
    if selection.is_some() || pending.queue.is_empty() {
        return;
    }

    let source = pending.queue.pop_front().unwrap();

    let player_upgrades = player_query
        .single()
        .map(|u| u.clone())
        .unwrap_or_default();

    let options = select_random_upgrades(&pool, &player_upgrades, 3);

    commands.insert_resource(UpgradeSelectionState {
        options,
        selected_index: 0,
        source,
    });

    next_state.set(GameState::UpgradeSelection);
}

fn check_pending_upgrades(
    mut pending: ResMut<PendingUpgradeSelections>,
    mut next_state: ResMut<NextState<GameState>>,
    pool: Res<UpgradePool>,
    player_query: Query<&PlayerUpgrades, With<PlayerTag>>,
    mut commands: Commands,
) {
    // clean up the old selection state, then check if more are queued
    commands.remove_resource::<UpgradeSelectionState>();

    if let Some(source) = pending.queue.pop_front() {
        let player_upgrades = player_query
            .single()
            .map(|u| u.clone())
            .unwrap_or_default();

        let options = select_random_upgrades(&pool, &player_upgrades, 3);

        commands.insert_resource(UpgradeSelectionState {
            options,
            selected_index: 0,
            source,
        });

        next_state.set(GameState::UpgradeSelection);
    }
}
