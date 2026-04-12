use crate::events::LevelUpEvent;
use crate::upgrades::ShowUpgradeSelectionEvent;
use crate::upgrades::UpgradeSource;
use bevy::prelude::*;

pub fn level_up_system(
    mut level_up_events: EventReader<LevelUpEvent>,
    mut upgrade_events: EventWriter<ShowUpgradeSelectionEvent>,
) {
    for _event in level_up_events.read() {
        upgrade_events.write(ShowUpgradeSelectionEvent {
            source: UpgradeSource::LevelUp,
        });
    }
}
