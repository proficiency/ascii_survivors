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
    if timer.0.finished() {
        if let Ok(mut player) = player_query.single_mut() {
            if let Ok(terminal) = terminal_query.single() {
                let size = terminal.size();
                let center_x = size[0] as i32 / 2;
                let center_y = size[1] as i32 / 2;

                let mut move_offset = IVec2::new(0, 0);
                for (_, gamepad) in &gamepad_input {
                    let left_stick = gamepad.left_stick();

                    // print!("Left Stick: x: {}, y: {}\r", left_stick.x, left_stick.y);
                    // print!(
                    //     "Left Stick abs: x: {}, y: {}\r",
                    //     left_stick.x.abs(),
                    //     left_stick.y.abs()
                    // );

                    const TOLERANCE: f32 = 0.35f32;
                    if left_stick.y < -TOLERANCE {
                        move_offset.y -= 1;
                    } else if left_stick.y > TOLERANCE {
                        move_offset.y += 1;
                    } else if left_stick.x < -TOLERANCE {
                        move_offset.x -= 1;
                    } else if left_stick.x > TOLERANCE {
                        move_offset.x += 1;
                    }
                }

                if keyboard_input.pressed(KeyCode::KeyW) {
                    move_offset.y += 1;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    move_offset.y -= 1;
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    move_offset.x -= 1;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    move_offset.x += 1;
                }
                // this is kinda weird
                camera_offset.0 -= move_offset.clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
                player.position = IVec2::new(center_x, center_y);
            }
        }
    }
}
