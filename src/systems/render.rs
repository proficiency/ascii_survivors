use crate::{objects::*, resources::*};
use crate::effects::Fireball as FireballEffect;
use crate::StatusEffect;
use bevy::prelude::*;
use bevy_ascii_terminal::string::TerminalString;
use bevy_ascii_terminal::*;

fn world_to_screen(world_position: IVec2, terminal_size: UVec2) -> IVec2 {
    IVec2::new(
        world_position.x,
        terminal_size[1] as i32 - 1 - world_position.y,
    )
}

pub struct ResourceBarConfig<'a> {
    pub resource_name: &'a str,
    pub filled_char: char,
    pub bar_length: usize,
    pub current_value: usize,
    pub max_value: usize,
    pub bar_color: Color,
    pub bar_x_position: usize,
    pub bar_y_position: usize,
}

pub fn draw_resource_bar(terminal_query: &mut Query<&mut Terminal>, config: ResourceBarConfig) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        let resource_ratio = if config.max_value > 0 {
            config.current_value as f32 / config.max_value as f32
        } else {
            0.0
        };
        let filled_length = (resource_ratio * config.bar_length as f32) as usize;

        let mut bar_content = String::new();
        for i in 0..config.bar_length {
            if i < filled_length {
                bar_content.push(config.filled_char);
            }
        }

        let mut bar_ts = TerminalString::from(bar_content);
        bar_ts.decoration.fg_color = Some(LinearRgba::from(config.bar_color));

        let formatted_resource_name = format!("{}:", config.resource_name);
        let formatted_name_length = formatted_resource_name.len();
        let formatted_bar_position = config.bar_x_position + formatted_name_length;
        if config.bar_length + formatted_bar_position <= terminal.size()[0] as usize {
            terminal.put_string(
                [config.bar_x_position, config.bar_y_position],
                formatted_resource_name,
            );
            terminal.put_string([formatted_bar_position, config.bar_y_position], bar_ts);
        }
    }
}

pub fn draw_survival_timer(
    terminal_query: &mut Query<&mut Terminal>,
    seconds_survived: f32,
    ruleset: &Ruleset,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        let timer_text = if seconds_survived >= ruleset.portal_spawn_time {
            "Portal Available".to_string()
        } else {
            format!("Time: {:.1}s", seconds_survived)
        };
        let text_length = timer_text.len() as i32;
        let terminal_width = terminal.size()[0] as i32;
        let x_position = (terminal_width - text_length) / 2;
        let x_position = std::cmp::max(0, x_position) as usize;

        let mut timer_ts = TerminalString::from(timer_text);
        timer_ts.decoration.fg_color =
            Some(LinearRgba::from(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)));
        terminal.put_string([x_position, 0], timer_ts);
    }
}

pub fn render_system(
    player_query: Query<(&Player, Option<&StatusEffect>)>,
    enemy_query: Query<&Enemy>,
    boss_query: Query<&Boss>,
    projectile_query: Query<&Projectile>,
    fireball_query: Query<&Projectile, With<Fireball>>,
    orb_query: Query<&Orb>,
    portal_query: Query<&Portal>,
    campfire_query: Query<&Campfire>,
    ember_query: Query<&Ember>,
    shop_npc_query: Query<&ShopNpc>,
    mut terminal_query: Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    level: Res<Level>,
) {
    draw_scene(
        player_query,
        enemy_query,
        boss_query,
        projectile_query,
        fireball_query,
        orb_query,
        portal_query,
        campfire_query,
        ember_query,
        shop_npc_query,
        &mut terminal_query,
        camera_offset,
        survival_timer.0.elapsed_secs(),
        &ruleset,
        level,
    );
}

pub fn draw_scene(
    player_query: Query<(&Player, Option<&StatusEffect>)>,
    enemy_query: Query<&Enemy>,
    boss_query: Query<&Boss>,
    projectile_query: Query<&Projectile>,
    fireball_query: Query<&Projectile, With<Fireball>>,
    orb_query: Query<&Orb>,
    portal_query: Query<&Portal>,
    campfire_query: Query<&Campfire>,
    ember_query: Query<&Ember>,
    shop_npc_query: Query<&ShopNpc>,
    terminal_query: &mut Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
    seconds_survived: f32,
    ruleset: &Ruleset,
    level: Res<Level>,
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

        // draw bosses
        for boss in boss_query.iter() {
            for segment in &boss.segments {
                let world_position = segment.position + camera_offset.0;
                let draw_position = world_to_screen(world_position, terminal_size);

                if terminal
                    .size()
                    .contains_point([draw_position.x, draw_position.y])
                {
                    let mut boss_char = TerminalString::from(segment.character.to_string());
                    boss_char.decoration.fg_color = Some(LinearRgba::from(segment.color));
                    terminal.put_string([draw_position.x, draw_position.y], boss_char);
                }
            }
        }

        // draw normal projectiles
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

        // draw fireballs
        for fireball in fireball_query.iter() {
            let world_position = fireball.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            // ensure fireball is within our viewport before drawing it
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut fireball_char = TerminalString::from("@");
                fireball_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgb(1.0, 0.3, 0.0)));
                terminal.put_string([draw_position.x, draw_position.y], fireball_char);
            }
        }

        // draw player
        if let Ok((player, status_effect)) = player_query.single() {
            // note: the player is assumed to always be in the center of our viewpoint
            let mut player_position = TerminalString::from("@");

            if let Some(effect) = status_effect {
                player_position.decoration.fg_color = Some(LinearRgba::from(effect.color));
            } else {
                player_position.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)));
            }

            terminal.put_string([player.position.x, player.position.y], player_position);
        }

        // draw portals
        for portal in portal_query.iter() {
            let world_position = portal.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut portal_char = TerminalString::from("P");
                portal_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgba(0.0, 1.0, 1.0, 1.0)));
                terminal.put_string([draw_position.x, draw_position.y], portal_char);
            }
        }
        // draw campfire
        for campfire in campfire_query.iter() {
            let world_position = campfire.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);
            let wood_position = IVec2::new(draw_position.x, draw_position.y + 1);
            if terminal
                .size()
                .contains_point([wood_position.x, wood_position.y])
            {
                let mut wood_char = TerminalString::from("=");
                wood_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgb(0.5, 0.25, 0.0))); // brown
                terminal.put_string([wood_position.x, wood_position.y], wood_char);
            }
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let (character, color) = campfire.get_current_visual();
                let mut campfire_char = TerminalString::from(character.to_string());
                campfire_char.decoration.fg_color = Some(LinearRgba::from(color));
                terminal.put_string([draw_position.x, draw_position.y], campfire_char);
            }
        }

        for ember in ember_query.iter() {
            let world_position = ember.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut ember_char = TerminalString::from(".");
                ember_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgb(1.0, 0.5, 0.0)));
                terminal.put_string([draw_position.x, draw_position.y], ember_char);
            }
        }

        // draw shop npcs
        for shop_npc in shop_npc_query.iter() {
            let world_position = shop_npc.position + camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);
            if terminal
                .size()
                .contains_point([draw_position.x, draw_position.y])
            {
                let mut npc_char = TerminalString::from("S");
                npc_char.decoration.fg_color =
                    Some(LinearRgba::from(Color::linear_rgb(0.0, 1.0, 1.0)));
                terminal.put_string([draw_position.x, draw_position.y], npc_char);
            }
        }

        // draw player info(hp bar, xp, etc)
        if let Ok((player, _)) = player_query.single() {
            draw_resource_bar(
                terminal_query,
                ResourceBarConfig {
                    resource_name: "HP",
                    filled_char: '#',
                    bar_length: 20,
                    current_value: player.health as usize,
                    max_value: player.max_health as usize,
                    bar_color: Color::linear_rgba(0.0, 1.0, 0.1, 1.0),
                    bar_x_position: 0,
                    bar_y_position: 0,
                },
            );
            draw_resource_bar(
                terminal_query,
                ResourceBarConfig {
                    resource_name: &format!("XP (Lvl {})", player.level),
                    filled_char: '#',
                    bar_length: 20,
                    current_value: player.experience as usize,
                    max_value: player.experience_to_next_level as usize,
                    bar_color: Color::linear_rgba(0.1, 0.25, 1.0, 1.0),
                    bar_x_position: 0,
                    bar_y_position: 49,
                },
            );
        }
        if matches!(level.as_ref(), Level::Survival) {
            draw_survival_timer(terminal_query, seconds_survived, ruleset);
        }
    }
}
