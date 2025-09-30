use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::objects::player::Player;
use crate::resources::camera::CameraOffset;

pub fn player_movement(
    mut player_query: Query<&mut Player>,
    gamepad_input: Query<(Entity, &Gamepad)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<crate::resources::timers::PlayerMovementTimer>,
    mut camera_offset: ResMut<CameraOffset>,
    terminal_query: Query<&Terminal>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished()
        && let Ok(mut player) = player_query.single_mut()
        && let Ok(terminal) = terminal_query.single()
    {
        let size = terminal.size();
        let center_x = size[0] as i32 / 2;
        let center_y = size[1] as i32 / 2;

        let mut move_offset = IVec2::new(0, 0);
        for (_, gamepad) in &gamepad_input {
            let left_stick = gamepad.left_stick();
            const TOLERANCE: f32 = 0.35f32;

            if left_stick.y < -TOLERANCE {
                move_offset.y -= 1;
            } 
            if left_stick.y > TOLERANCE {
                move_offset.y += 1;
            } 
            if left_stick.x < -TOLERANCE {
                move_offset.x -= 1;
            } 
            if left_stick.x > TOLERANCE {
                move_offset.x += 1;
            }

            // D-Pad movement
            if gamepad.pressed(GamepadButton::DPadUp) {
                move_offset.y += 1;
            }
            if gamepad.pressed(GamepadButton::DPadDown) {
                move_offset.y -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadLeft) {
                move_offset.x -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadRight) {
                move_offset.x += 1;
            }
        }

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            move_offset.y += 1;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            move_offset.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            move_offset.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            move_offset.x += 1;
        }
        
        // this is kinda weird
        camera_offset.0 -= move_offset.clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
        player.position = IVec2::new(center_x, center_y);
    }
}
