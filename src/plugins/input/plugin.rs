use crate::{
    events::*,
    plugins::core::GameSet,
    resources::{InteractionTimer, PlayerMovementTimer, UiNavRepeatTimer},
};
use bevy::input::gamepad::{GamepadConnection, GamepadEvent};
use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Resource)]
struct ActiveGamepad(Entity);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiNavRepeatTimer>()
            .add_event::<WalkEvent>()
            .add_event::<UiActionEvent>()
            .add_event::<InteractEvent>()
            .add_systems(
                Update,
                (
                    select_active_gamepad,
                    dispatch_walk_events,
                    dispatch_ui_events,
                    dispatch_interact_events,
                )
                    .chain()
                    .in_set(GameSet::Input),
            );
    }
}
// todo: eventually we should prompt the user before the game starts to select a gamepad if multiple are connected
// maybe we could narrow it down by checking if the string contains "Xbox" or "PlayStation"?
fn select_active_gamepad(
    mut commands: Commands,
    my_gamepad: Option<Res<ActiveGamepad>>,
    mut gamepad_events: EventReader<GamepadEvent>,
) {
    for event in gamepad_events.read() {
        let GamepadEvent::Connection(connection_event) = event else {
            continue;
        };

        match &connection_event.connection {
            GamepadConnection::Connected { name, .. } => {
                info!("[Input] Gamepad '{}' connected", name);

                if my_gamepad.is_none() {
                    commands.insert_resource(ActiveGamepad(connection_event.gamepad));
                    info!("[Input] Gamepad '{}' set as primary input device", name);
                }
            }

            GamepadConnection::Disconnected => {
                // todo: print name if we can get it
                info!(
                    "[Input] Dropping active gamepad: {:?}",
                    connection_event.gamepad
                );

                if let Some(ActiveGamepad(old_id)) = my_gamepad.as_deref()
                    && *old_id == connection_event.gamepad
                {
                    commands.remove_resource::<ActiveGamepad>();
                }
            }
        }
    }
}

fn sample_walk_direction(
    active_gamepad: Option<&ActiveGamepad>,
    keyboard_input: &ButtonInput<KeyCode>,
    gamepad_input: &Query<(Entity, &Gamepad)>,
) -> IVec2 {
    let mut direction = IVec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y -= 1;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y += 1;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1;
    }

    if let Some(active_gamepad) = active_gamepad {
        for (entity, gamepad) in gamepad_input.iter() {
            // only sample from the active gamepad
            if active_gamepad.0 != entity {
                continue;
            }

            let left_stick = gamepad.left_stick();
            const DEADZONE: f32 = 0.35f32;

            let x = left_stick.x;
            let y = -left_stick.y; // invert so up is negative like keyboard

            if x.abs() >= DEADZONE {
                direction.x += x.signum() as i32;
            }
            if y.abs() >= DEADZONE {
                direction.y += y.signum() as i32;
            }

            if gamepad.pressed(GamepadButton::DPadUp) {
                direction.y -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadDown) {
                direction.y += 1;
            }
            if gamepad.pressed(GamepadButton::DPadLeft) {
                direction.x -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadRight) {
                direction.x += 1;
            }

            break;
        }
    }

    direction.clamp(IVec2::new(-1, -1), IVec2::new(1, 1))
}

fn dispatch_walk_events(
    mut walk_events: EventWriter<WalkEvent>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMovementTimer>,
    active_gamepad: Option<Res<ActiveGamepad>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<(Entity, &Gamepad)>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let dir = sample_walk_direction(active_gamepad.as_deref(), &keyboard_input, &gamepad_input);
    if dir != IVec2::ZERO {
        walk_events.write(WalkEvent { direction: dir });
    }
}

fn dispatch_ui_events(
    mut ui_events: EventWriter<UiActionEvent>,
    active_gamepad: Option<Res<ActiveGamepad>>,
    time: Res<Time>,
    mut repeat: ResMut<UiNavRepeatTimer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    repeat.0.tick(time.delta());

    let dir = sample_walk_direction(active_gamepad.as_deref(), &keyboard, &gamepads);
    let nav_pressed = dir != IVec2::ZERO;

    if nav_pressed && repeat.0.finished() {
        ui_events.write(UiActionEvent::Navigate(dir));
        info!("[Input] Dispatched UiActionEvent::Navigate({:?})", dir);
        repeat.0.reset();
    }

    let mut submit = keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);
    let mut cancel = keyboard.just_pressed(KeyCode::Escape);
    let mut info = keyboard.just_pressed(KeyCode::Insert);

    if let Some(active_gamepad) = active_gamepad.as_deref() {
        for (entity, gamepad) in gamepads.iter() {
            if active_gamepad.0 != entity {
                continue;
            }

            if gamepad.just_pressed(GamepadButton::South)
                || gamepad.just_pressed(GamepadButton::Start)
            {
                submit = true;
            }
            if gamepad.just_pressed(GamepadButton::East)
                || gamepad.just_pressed(GamepadButton::West)
            {
                cancel = true;
            }
            if gamepad.just_pressed(GamepadButton::Select)
                || gamepad.just_pressed(GamepadButton::Mode)
            {
                info = true;
            }

            break;
        }
    }

    if submit {
        ui_events.write(UiActionEvent::Submit);
        info!("[Input] Dispatched UiActionEvent::Submit");
    }
    if cancel {
        ui_events.write(UiActionEvent::Cancel);
        info!("[Input] Dispatched UiActionEvent::Cancel");
    }
    if info {
        ui_events.write(UiActionEvent::Info);
        info!("[Input] Dispatched UiActionEvent::Info");
    }
}

fn dispatch_interact_events(
    mut interact_events: EventWriter<InteractEvent>,
    mut interaction_timer: ResMut<InteractionTimer>,
    time: Res<Time>,
    active_gamepad: Option<Res<ActiveGamepad>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    interaction_timer.0.tick(time.delta());
    if !interaction_timer.0.finished() {
        return;
    }

    let mut interacted = keyboard.just_pressed(KeyCode::KeyE);

    if let Some(active_gamepad) = active_gamepad.as_deref() {
        for (entity, gamepad) in gamepads.iter() {
            if active_gamepad.0 != entity {
                continue;
            }

            if gamepad.just_pressed(GamepadButton::South)
                || gamepad.just_pressed(GamepadButton::Start)
            {
                interacted = true;
            }

            break;
        }
    }

    if interacted {
        interact_events.write(InteractEvent);
        interaction_timer.0.reset();
        info!("[Input] Dispatched InteractEvent");
    }
}
