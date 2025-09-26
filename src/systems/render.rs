use crate::{CameraOffset, Enemy, Player, Projectile};
use bevy::prelude::*;
use bevy_ascii_terminal::string::TerminalString;
use bevy_ascii_terminal::*;

pub fn draw_scene(
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    projectile_query: Query<&Projectile>,
    mut terminal_query: Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        terminal.clear();

        // draw player
        if let Ok(player) = player_query.single() {
            // note: the player is assumed to always be in the center of our viewpoint
            terminal.put_char([player.position.x, player.position.y], '@');
        }

        // draw enemies
        for enemy in enemy_query.iter() {
            let draw_position = enemy.position + camera_offset.0;

            // ensure entity is within our viewport before drawing it
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                terminal.put_char([draw_position.x, draw_position.y], 'd');
            }
        }

        // draw projectiles
        for projectile in projectile_query.iter() {
            let draw_position = projectile.position + camera_offset.0;

            // ensure projectile is within our viewport before drawing it
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                terminal.put_char([draw_position.x, draw_position.y], '*');
            }
        }

        // draw player info(hp bar, etc)
        if let Ok(player) = player_query.single() {
            let health_ratio = player.health / player.max_health;
            let bar_length = 20;
            let filled_length = (health_ratio * bar_length as f32) as usize;

            let healthbar_base = String::from("HP:");
            let mut healthbar_content = String::new();
            for i in 0..bar_length {
                if i < filled_length {
                    healthbar_content.push('#');
                } else {
                    healthbar_content.push('-');
                }
            }

            let mut healthbar_ts = TerminalString::from(healthbar_content);
            healthbar_ts.decoration.fg_color = Some(Color::linear_rgb(0.0, 1.0, 0.0).into());

            // position the healthbar in the top-left corner, ensuring it fits
            if bar_length + 6 <= terminal.size()[0] as usize {
                terminal.put_string([0, 0], healthbar_base);
                terminal.put_string([4, 0], healthbar_ts);
            }
        }
    }
}
