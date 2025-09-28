use crate::{CameraOffset, Enemy, Orb, Player, Projectile};
use bevy::prelude::*;
use bevy_ascii_terminal::string::TerminalString;
use bevy_ascii_terminal::*;

fn world_to_screen(world_position: IVec2, terminal_size: UVec2) -> IVec2 {
    IVec2::new(
        world_position.x,
        terminal_size[1] as i32 - 1 - world_position.y,
    )
}

pub fn draw_resource_bar(
    terminal_query: &mut Query<&mut Terminal>,
    resource_name: &str,
    filled_char: char,
    bar_length: usize,
    current_value: usize,
    max_value: usize,
    bar_color: Color,
    bar_x_position: usize,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        let resource_ratio = if max_value > 0 {
            current_value as f32 / max_value as f32
        } else {
            0.0
        };
        let filled_length = (resource_ratio * bar_length as f32) as usize;

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
    orb_query: Query<&Orb>,
    mut terminal_query: Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        terminal.clear();

        let terminal_size = terminal.size();

        // draw orbs
        for orb in orb_query.iter() {
            let world_position = orb.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut orb_char = TerminalString::from("o");
                orb_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgba(0.8, 0.2, 0.8, 1.0)));
                terminal.put_string([draw_position.x, draw_position.y], orb_char);
            }
        }

        // draw enemies
        for enemy in enemy_query.iter() {
            let world_position = enemy.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut enemy_char = TerminalString::from("d");
                enemy_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)));
                terminal.put_string([draw_position.x, draw_position.y], enemy_char);
            }
        }

        // draw projectiles
        for projectile in projectile_query.iter() {
            let world_position = projectile.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            // ensure projectile is within our viewport before drawing it
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut projectile_char = TerminalString::from("*");
                projectile_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgba(1.0, 0.7, 0.0, 1.0)));
                terminal.put_string([draw_position.x, draw_position.y], projectile_char);
            }
        }

        // draw player
        if let Ok(player) = player_query.single() {
            // note: the player is assumed to always be in the center of our viewpoint
            let mut player_position = TerminalString::from("@");
            player_position.decoration.fg_color =
                Some(LinearRgba::from(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)));
            terminal.put_string([player.position.x, player.position.y], player_position);
        }

        // draw player info(hp bar, xp, etc)
        if let Ok(player) = player_query.single() {
        draw_resource_bar(
            &mut terminal_query,
            "HP",
            '#',
            20,
            player.health as usize,
            player.max_health as usize,
            Color::linear_rgba(0.0, 1.0, 0.1, 1.0),
            0,
        );
        draw_resource_bar(
            &mut terminal_query,
            &format!("XP (Lvl {})", player.level),
            '#',
            20,
            player.experience as usize,
            player.experience_to_next_level as usize,
            Color::linear_rgba(0.1, 0.25, 1.0, 1.0),
            30,
        );
        }
    }
}
