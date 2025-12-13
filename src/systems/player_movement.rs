use crate::{maps::*, objects::*, resources::*};
use bevy::prelude::*;
use bevy_ascii_terminal::*;

pub fn player_movement(
    mut player_query: Query<&mut Player>,
    gamepad_input: Query<(Entity, &Gamepad)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMovementTimer>,
    mut camera_offset: ResMut<CameraOffset>,
    terminal_query: Query<&Terminal>,
    scene_lock: Res<SceneLock>,
    map: Option<Res<Map>>,
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
            const DEADZONE: f32 = 0.35f32;

            let x = left_stick.x;
            let y = -left_stick.y; // invert so up is negative like keyboard

            if x.abs() >= DEADZONE {
                move_offset.x += x.signum() as i32;
            }
            if y.abs() >= DEADZONE {
                move_offset.y += y.signum() as i32;
            }

            // D-pad movement
            if gamepad.pressed(GamepadButton::DPadUp) {
                move_offset.y -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadDown) {
                move_offset.y += 1;
            }
            if gamepad.pressed(GamepadButton::DPadLeft) {
                move_offset.x -= 1;
            }
            if gamepad.pressed(GamepadButton::DPadRight) {
                move_offset.x += 1;
            }
        }

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            move_offset.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            move_offset.y += 1;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            move_offset.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            move_offset.x += 1;
        }

        let clamped = move_offset.clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
        if clamped == IVec2::ZERO {
            return;
        }

        let world_delta = IVec2::new(clamped.x, -clamped.y);

        if let Some(map) = &map {
            let (position, world_position, update_camera_offset) = if scene_lock.0 {
                let new_pos = player.position + clamped;

                let wish_move =
                    IVec2::new(new_pos.x, size[1] as i32 - 1 - new_pos.y) + camera_offset.0;
                (new_pos, wish_move, false)
            } else {
                let center = IVec2::new(center_x, center_y);
                let wish_move: IVec2 = player.world_position + world_delta;
                (center, wish_move, true)
            };

            if map.is_walkable(world_position.x, world_position.y) {
                player.position = position;
                player.world_position = world_position;

                if update_camera_offset {
                    camera_offset.0 = world_position - IVec2::new(center_x, center_y);
                }
            }
        }
    }
}
