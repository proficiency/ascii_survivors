use crate::{CameraOffset, Enemy, Player, Projectile};
use bevy::prelude::*;
use bevy_ascii_terminal::string::TerminalString;
use bevy_ascii_terminal::*;

pub fn draw_resource_bar(
    mut terminal_query: &mut Query<&mut Terminal>, // a query to the terminal resource
    resource_name: &str,                           // name of the resource (e.g., "HP", "XP", "MANA")
    filled_char: char,                             // character to represent filled portion of the resource bar
    bar_length: usize,                             // total length of the resource bar
    current_value: usize,                          // current value of the resource
    max_value: usize,                              // maximum value of the resource pool
    bar_color: Color,                              // color
    bar_x_position: usize,                         // placeholder: x position of the bar (not used in future implementation)
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        let resource_ratio = current_value / max_value;
        let filled_length = (resource_ratio as f32 * bar_length as f32) as usize;

        let mut bar_content = String::new();
        for i in 0..bar_length {
            if i < filled_length {
                bar_content.push(filled_char);
            }
        }

        let mut bar_ts = TerminalString::from(bar_content);
        bar_ts.decoration.fg_color = Some(LinearRgba::from(bar_color));

        // position the bar in the top-left corner, ensuring it fits
        let formatted_resource_name = format!("{}:", resource_name);
        let formatted_name_length = formatted_resource_name.len();
        let formatted_bar_position = bar_x_position + formatted_name_length;
        if bar_length + formatted_bar_position <= terminal.size()[0] as usize {
            terminal.put_string([bar_x_position, 0], formatted_resource_name);
            terminal.put_string([formatted_bar_position, 0], bar_ts);
        }
    }
}

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

        // draw player info(hp bar, xp, etc)
        let player_query = player_query.single().unwrap();
        draw_resource_bar(&mut terminal_query, "HP", '#', 20, player_query.health as usize, player_query.max_health as usize, Color::linear_rgba(0.0, 1.0, 0.1, 1.0), 0);
        draw_resource_bar(&mut terminal_query, "XP", '#', 20, 30, 100, Color::linear_rgba(0.1, 0.25, 1.0, 1.0), 30);
    }
}
