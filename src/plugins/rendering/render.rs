use crate::{effects::StatusEffect, maps::*, objects::*, resources::*, spells::Arcanum};
use bevy::prelude::*;

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

pub fn draw_resource_bar(frame: &mut AsciiFrame, config: ResourceBarConfig) {
    let resource_ratio = if config.max_value > 0 {
        config.current_value as f32 / config.max_value as f32
    } else {
        0.0
    };
    let filled_length = (resource_ratio * config.bar_length as f32) as usize;

    let formatted_resource_name = format!("{}:", config.resource_name);
    let formatted_name_length = formatted_resource_name.len();
    let formatted_bar_position = config.bar_x_position + formatted_name_length;
    if config.bar_length + formatted_bar_position <= frame.size.x as usize {
        frame.put_string(
            IVec2::new(config.bar_x_position as i32, config.bar_y_position as i32),
            formatted_resource_name.as_str(),
            Color::WHITE,
            Color::NONE,
        );
        for i in 0..config.bar_length {
            if i < filled_length {
                frame.put_char(
                    IVec2::new(
                        (formatted_bar_position + i) as i32,
                        config.bar_y_position as i32,
                    ),
                    config.filled_char,
                    config.bar_color,
                    Color::NONE,
                );
            }
        }
    }
}

pub fn draw_survival_timer(frame: &mut AsciiFrame, seconds_survived: f32, ruleset: &Ruleset) {
    let timer_text = if seconds_survived >= ruleset.portal_spawn_time {
        "Portal Available".to_string()
    } else {
        format!("Time: {:.1}s", seconds_survived)
    };
    let text_length = timer_text.len() as i32;
    let terminal_width = frame.size.x as i32;
    let x_position = (terminal_width - text_length) / 2;
    let x_position = std::cmp::max(0, x_position) as i32;

    frame.put_string(
        IVec2::new(x_position, 0),
        timer_text.as_str(),
        Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
        Color::NONE,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn render_system(
    player_query: Query<
        (
            &GridPosition,
            &Health,
            &Experience,
            &Arcanum,
            Option<&StatusEffect>,
        ),
        With<PlayerTag>,
    >,
    enemy_query: Query<&Enemy>,
    boss_query: Query<&Boss>,
    projectile_query: Query<(&Projectile, Option<&Fireball>)>,
    orb_query: Query<&Orb>,
    upgrade_orb_query: Query<&UpgradeOrb>,
    portal_query: Query<&Portal>,
    campfire_query: Query<&Campfire>,
    ember_query: Query<&Ember>,
    shop_npc_query: Query<&ShopNpc>,
    mut frame: ResMut<AsciiFrame>,
    camera_offset: Res<CameraOffset>,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    level: Res<Level>,
    map: Option<Res<Map>>,
) {
    draw_scene(
        player_query,
        enemy_query,
        boss_query,
        projectile_query,
        orb_query,
        upgrade_orb_query,
        portal_query,
        campfire_query,
        ember_query,
        shop_npc_query,
        &mut frame,
        camera_offset,
        survival_timer.0.elapsed_secs(),
        &ruleset,
        level,
        map,
    );
}

fn draw_map(frame: &mut AsciiFrame, map: &Map, camera_offset: IVec2, terminal_size: UVec2) {
    for x in 0..map.width {
        for y in 0..map.height {
            let world_position = IVec2::new(x as i32, y as i32) - camera_offset;
            let draw_position = world_to_screen(world_position, terminal_size);

            if frame.contains(draw_position)
                && let Some(tile) = map.get_tile(x as i32, y as i32)
                && tile.explored
            {
                frame.put_char(
                    draw_position,
                    tile.tile_type.to_char(),
                    tile.tile_type.to_color(),
                    tile.tile_type.to_bg_color(),
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
pub fn draw_scene(
    player_query: Query<
        (
            &GridPosition,
            &Health,
            &Experience,
            &Arcanum,
            Option<&StatusEffect>,
        ),
        With<PlayerTag>,
    >,
    enemy_query: Query<&Enemy>,
    boss_query: Query<&Boss>,
    projectile_query: Query<(&Projectile, Option<&Fireball>)>,
    orb_query: Query<&Orb>,
    upgrade_orb_query: Query<&UpgradeOrb>,
    portal_query: Query<&Portal>,
    campfire_query: Query<&Campfire>,
    ember_query: Query<&Ember>,
    shop_npc_query: Query<&ShopNpc>,
    frame: &mut AsciiFrame,
    camera_offset: Res<CameraOffset>,
    seconds_survived: f32,
    ruleset: &Ruleset,
    level: Res<Level>,
    map: Option<Res<Map>>,
) {
    frame.clear();
    let terminal_size = frame.size;

    if let Some(map) = map {
        draw_map(frame, &map, camera_offset.0, terminal_size);
    }

    // draw orbs
    for orb in orb_query.iter() {
        let world_position = orb.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                'o',
                Color::linear_rgba(0.8, 0.2, 0.8, 1.0),
                Color::NONE,
            );
        }
    }

    for orb in upgrade_orb_query.iter() {
        let world_position = orb.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                'O',
                Color::linear_rgb(1.0, 0.85, 0.0),
                Color::NONE,
            );
        }
    }

    // draw enemies
    for enemy in enemy_query.iter() {
        let world_position = enemy.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                'd',
                Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
                Color::NONE,
            );
        }
    }

    // draw bosses
    for boss in boss_query.iter() {
        for segment in &boss.segments {
            let world_position = segment.position - camera_offset.0;
            let draw_position = world_to_screen(world_position, terminal_size);

            if frame.contains(draw_position) {
                frame.put_char(draw_position, segment.character, segment.color, Color::NONE);
            }
        }
    }

    for (projectile, fireball) in projectile_query.iter() {
        let world_position = projectile.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            let (ch, color) = if fireball.is_some() {
                ('@', Color::linear_rgb(1.0, 0.3, 0.0))
            } else {
                ('*', Color::linear_rgba(1.0, 0.7, 0.0, 1.0))
            };
            frame.put_char(draw_position, ch, color, Color::NONE);
        }
    }

    // draw player
    if let Ok((pos, _, _, _, status_effect)) = player_query.single() {
        // note: the player is assumed to always be in the center of our viewpoint
        let color: Color = status_effect
            .map(|effect| effect.color)
            .unwrap_or_else(|| Color::linear_rgba(1.0, 1.0, 1.0, 1.0));
        frame.put_char(pos.tile, '@', color, Color::NONE);
    }

    // draw portals
    for portal in portal_query.iter() {
        let world_position = portal.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                'P',
                Color::linear_rgba(0.0, 1.0, 1.0, 1.0),
                Color::NONE,
            );
        }
    }

    // draw campfire
    for campfire in campfire_query.iter() {
        let world_position = campfire.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);
        let wood_position = IVec2::new(draw_position.x, draw_position.y + 1);
        if frame.contains(wood_position) {
            frame.put_char(
                wood_position,
                '=',
                Color::linear_rgb(0.5, 0.25, 0.0),
                Color::NONE,
            );
        }
        if frame.contains(draw_position) {
            let (character, color) = campfire.get_current_visual();
            frame.put_char(draw_position, character, color, Color::NONE);
        }
    }

    for ember in ember_query.iter() {
        let world_position = ember.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);

        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                '.',
                Color::linear_rgb(1.0, 0.5, 0.0),
                Color::NONE,
            );
        }
    }

    // draw shop npcs
    for shop_npc in shop_npc_query.iter() {
        let world_position = shop_npc.position - camera_offset.0;
        let draw_position = world_to_screen(world_position, terminal_size);
        if frame.contains(draw_position) {
            frame.put_char(
                draw_position,
                'S',
                Color::linear_rgb(0.0, 1.0, 1.0),
                Color::NONE,
            );
        }
    }

    // draw player info(hp bar, xp, etc)
    if let Ok((_, health, xp, arcanum, _)) = player_query.single() {
        let height = frame.size.y as i32;
        let base_y = (height - 3).max(0) as usize;
        // 1 bar char per 5 hp/mana/xp, clamped to fit on screen
        let max_bar_len = (frame.size.x as usize).saturating_sub(15);
        let health_bar_len = (health.max as usize / 5).clamp(20, max_bar_len);
        let mana_bar_len = (arcanum.max_mana as usize / 5).clamp(20, max_bar_len);
        let xp_bar_len = (xp.to_next as usize / 5).clamp(20, max_bar_len);

        draw_resource_bar(
            frame,
            ResourceBarConfig {
                resource_name: "Health",
                filled_char: '#',
                bar_length: health_bar_len,
                current_value: health.current as usize,
                max_value: health.max as usize,
                bar_color: Color::linear_rgba(0.0, 1.0, 0.1, 1.0),
                bar_x_position: 0,
                bar_y_position: base_y,
            },
        );
        draw_resource_bar(
            frame,
            ResourceBarConfig {
                resource_name: "Mana",
                filled_char: '#',
                bar_length: mana_bar_len,
                current_value: arcanum.mana as usize,
                max_value: arcanum.max_mana as usize,
                bar_color: Color::linear_rgba(0.15, 0.45, 1.0, 1.0),
                bar_x_position: 0,
                bar_y_position: base_y + 1,
            },
        );
        draw_resource_bar(
            frame,
            ResourceBarConfig {
                resource_name: &format!("XP (Lvl {})", xp.level),
                filled_char: '#',
                bar_length: xp_bar_len,
                current_value: xp.current as usize,
                max_value: xp.to_next as usize,
                bar_color: Color::linear_rgba(0.1, 0.25, 1.0, 1.0),
                bar_x_position: 0,
                bar_y_position: base_y + 2,
            },
        );
    }

    if matches!(level.as_ref(), Level::Survival) {
        draw_survival_timer(frame, seconds_survived, ruleset);
    }
}
