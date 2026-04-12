use bevy::prelude::*;

use crate::events::UiActionEvent;
use crate::resources::GameState;
use crate::upgrades::*;

// debounce timer so held keys don't scroll the menu every frame
#[derive(Resource)]
pub struct UpgradeNavTimer(pub Timer);

impl Default for UpgradeNavTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.15, TimerMode::Once))
    }
}

pub fn upgrade_selection_input(
    mut ui_events: EventReader<UiActionEvent>,
    mut selection: ResMut<UpgradeSelectionState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut upgrade_events: EventWriter<UpgradeSelectedEvent>,
    mut nav_timer: ResMut<UpgradeNavTimer>,
    time: Res<Time>,
) {
    nav_timer.0.tick(time.delta());

    for event in ui_events.read() {
        match event {
            UiActionEvent::Navigate(dir) => {
                let option_count = selection.options.len();
                if option_count == 0 || !nav_timer.0.finished() {
                    continue;
                }

                // y > 0 = S/down, y < 0 = W/up
                if dir.y > 0 {
                    selection.selected_index = (selection.selected_index + 1) % option_count;
                } else if dir.y < 0 {
                    selection.selected_index =
                        (selection.selected_index + option_count - 1) % option_count;
                }

                nav_timer.0.reset();
            }
            UiActionEvent::Submit => {
                if selection.options.is_empty() {
                    next_state.set(GameState::Game);
                    continue;
                }
                let chosen = selection.options[selection.selected_index].clone();
                upgrade_events.write(UpgradeSelectedEvent { upgrade: chosen });
                next_state.set(GameState::Game);
            }
            _ => {}
        }
    }
}
