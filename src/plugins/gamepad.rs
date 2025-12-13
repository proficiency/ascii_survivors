use bevy::input::gamepad::{GamepadConnection, GamepadEvent};
use bevy::prelude::*;

pub struct GamepadPlugin;

// todo: dispatching and handling of events to systems from kb/m and gamepad inputs
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_gamepad_event);
    }
}

fn on_gamepad_event(mut gamepad_events: EventReader<GamepadEvent>) {
    for event in gamepad_events.read() {
        // we only care about connection events
        let GamepadEvent::Connection(connection_event) = event else {
            continue;
        };

        match &connection_event.connection {
            GamepadConnection::Connected { name, .. } => {
                info!("[Gamepad] '{}' connected", name);
            }

            GamepadConnection::Disconnected => {}
        }
    }
}
