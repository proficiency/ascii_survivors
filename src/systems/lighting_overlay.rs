use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
    sprite::Sprite,
};
use bevy_ascii_terminal::Terminal;

use crate::{
    objects::{Boss, Campfire, Enemy, LightEmitter, Player, ShopNpc},
    resources::{CameraOffset, LightingOverlay},
};

const LIGHTING_PIXEL_SCALE: u32 = 4;

pub fn setup_lighting_overlay(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    terminal_query: Query<&Terminal>,
) {
    let terminal = terminal_query
        .single()
        .expect("terminal entity should exist before lighting overlay setup");
    let size = terminal.size();
    let logical_size = UVec2::new(size[0] as u32, size[1] as u32);
    let texture_size = logical_size * LIGHTING_PIXEL_SCALE;
    let pixel_count = (texture_size.x * texture_size.y) as usize;

    let mut image = Image::new_fill(
        Extent3d {
            width: texture_size.x,
            height: texture_size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.texture_descriptor.usage |= TextureUsages::COPY_DST;

    let handle = images.add(image);
    let ambient = LinearRgba::from(Color::srgba(0.01, 0.01, 0.02, 0.35));

    commands.insert_resource(LightingOverlay {
        handle: handle.clone(),
        size: logical_size,
        texture_size,
        pixel_scale: LIGHTING_PIXEL_SCALE,
        ambient_color: ambient,
        buffer: vec![ambient; pixel_count],
    });

    let mut overlay_sprite = Sprite::from_image(handle.clone());
    overlay_sprite.custom_size = Some(Vec2::new(size[0] as f32, size[1] as f32));

    commands.spawn((
        overlay_sprite,
        Transform::from_xyz(size[0] as f32 / 2.0, size[1] as f32 / 2.0, 5.0),
        Name::new("LightingOverlay"),
    ));
}

pub fn update_lighting_overlay(
    mut overlay: ResMut<LightingOverlay>,
    mut images: ResMut<Assets<Image>>,
    camera_offset: Res<CameraOffset>,
    player_lights: Query<(&Player, &LightEmitter)>,
    campfire_lights: Query<(&Campfire, &LightEmitter)>,
    player_occluders: Query<&Player>,
    shop_occluders: Query<&ShopNpc>,
    enemy_occluders: Query<&Enemy>,
    boss_occluders: Query<&Boss>,
) {
    let Some(image) = images.get_mut(&overlay.handle) else {
        return;
    };

    overlay.clear();

    let width = overlay.texture_size.x as usize;
    let height = overlay.texture_size.y as usize;
    let overlay_size = overlay.size;
    let overlay_scale = overlay.pixel_scale;

    for (player, emitter) in player_lights.iter() {
        let pos = screen_position(
            player.world_position,
            camera_offset.0,
            overlay_size,
            overlay_scale,
        );
        apply_light_basic(
            pos,
            emitter,
            &mut overlay.buffer,
            width,
            height,
            overlay_scale,
        );
    }

    let occluders = gather_occluders(
        &player_occluders,
        &shop_occluders,
        &enemy_occluders,
        &boss_occluders,
        overlay_size,
        overlay_scale,
        camera_offset.0,
    );

    for (campfire, emitter) in campfire_lights.iter() {
        let pos = screen_position(
            campfire.position,
            camera_offset.0,
            overlay_size,
            overlay_scale,
        );

        let radius_pixels = emitter.radius * overlay_scale as f32;
        let relevant_occluders: Vec<LightOccluder> = occluders
            .iter()
            .filter(|occluder| {
                (occluder.center - pos).length_squared()
                    <= (radius_pixels + occluder.radius).powi(2)
            })
            .cloned()
            .collect();

        if relevant_occluders.is_empty() {
            apply_light_basic(
                pos,
                emitter,
                &mut overlay.buffer,
                width,
                height,
                overlay_scale,
            );
        } else {
            apply_light_with_shadows(
                pos,
                emitter,
                &relevant_occluders,
                &mut overlay.buffer,
                width,
                height,
                overlay_scale,
            );
        }
    }

    if let Some(data) = image.data.as_mut() {
        for (idx, color) in overlay.buffer.iter().enumerate() {
            let data_index = idx * 4;
            data[data_index] = (color.red.clamp(0.0, 1.0) * 255.0) as u8;
            data[data_index + 1] = (color.green.clamp(0.0, 1.0) * 255.0) as u8;
            data[data_index + 2] = (color.blue.clamp(0.0, 1.0) * 255.0) as u8;
            data[data_index + 3] = (color.alpha.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
}

fn screen_position(world: IVec2, camera_offset: IVec2, size: UVec2, pixel_scale: u32) -> Vec2 {
    let adjusted = world + camera_offset;
    let x = adjusted.x as f32 + 0.5;
    let y = size.y as f32 - adjusted.y as f32 - 1.0 + 0.5;
    Vec2::new(x * pixel_scale as f32, y * pixel_scale as f32)
}

fn blend_color(base: LinearRgba, light: LinearRgba, weight: f32) -> LinearRgba {
    LinearRgba::new(
        (base.red + light.red * weight).clamp(0.0, 1.0),
        (base.green + light.green * weight).clamp(0.0, 1.0),
        (base.blue + light.blue * weight).clamp(0.0, 1.0),
        (base.alpha + light.alpha * weight).clamp(0.0, 1.0),
    )
}

#[derive(Clone)]
struct LightOccluder {
    center: Vec2,
    radius: f32,
}

fn gather_occluders(
    player_query: &Query<&Player>,
    shop_query: &Query<&ShopNpc>,
    enemy_query: &Query<&Enemy>,
    boss_query: &Query<&Boss>,
    overlay_size: UVec2,
    overlay_scale: u32,
    camera_offset: IVec2,
) -> Vec<LightOccluder> {
    let mut occluders = Vec::new();

    if let Some(player) = player_query.iter().next() {
        occluders.push(LightOccluder {
            center: screen_position(
                player.world_position,
                camera_offset,
                overlay_size,
                overlay_scale,
            ),
            radius: 0.85 * overlay_scale as f32,
        });
    }

    for shop in shop_query.iter() {
        occluders.push(LightOccluder {
            center: screen_position(shop.position, camera_offset, overlay_size, overlay_scale),
            radius: 0.7 * overlay_scale as f32,
        });
    }

    for enemy in enemy_query.iter() {
        occluders.push(LightOccluder {
            center: screen_position(enemy.position, camera_offset, overlay_size, overlay_scale),
            radius: 0.65 * overlay_scale as f32,
        });
    }

    for boss in boss_query.iter() {
        for segment in &boss.segments {
            occluders.push(LightOccluder {
                center: screen_position(
                    segment.position,
                    camera_offset,
                    overlay_size,
                    overlay_scale,
                ),
                radius: 0.75 * overlay_scale as f32,
            });
        }
    }

    occluders
}

fn occlusion_factor(point: Vec2, emitter: Vec2, occluders: &[LightOccluder]) -> f32 {
    let to_point = point - emitter;
    let point_len_sq = to_point.length_squared();
    if point_len_sq <= f32::EPSILON {
        return 0.0;
    }
    let point_len = point_len_sq.sqrt();

    let mut occlusion: f32 = 0.0;

    for occluder in occluders {
        let to_occluder = occluder.center - emitter;
        let occ_len = to_occluder.length();

        if occ_len <= f32::EPSILON || occ_len >= point_len {
            continue;
        }

        let projection = to_occluder.dot(to_point) / point_len_sq;
        if projection <= 0.0 || projection >= 1.05 {
            continue;
        }

        let nearest = emitter + to_point * projection;
        let distance = (occluder.center - nearest).length();

        let softness = occluder.radius * 1.5;
        if distance >= softness {
            continue;
        }

        let edge_mix = 1.0 - (distance / softness).clamp(0.0, 1.0);
        let depth_mix = 1.0 - (occ_len / point_len).clamp(0.0, 1.0);
        let influence = (edge_mix.powf(1.8) * depth_mix.powf(1.2)).clamp(0.0, 1.0);

        occlusion = occlusion.max(influence);
    }

    occlusion
}

fn apply_light_basic(
    screen_pos: Vec2,
    emitter: &LightEmitter,
    buffer: &mut [LinearRgba],
    width: usize,
    height: usize,
    overlay_scale: u32,
) {
    let radius_pixels = emitter.radius * overlay_scale as f32;
    if radius_pixels <= 1.0 {
        return;
    }

    let min_x = (screen_pos.x - radius_pixels).floor().max(0.0) as usize;
    let max_x = (screen_pos.x + radius_pixels)
        .ceil()
        .min(width as f32 - 1.0) as usize;
    let min_y = (screen_pos.y - radius_pixels).floor().max(0.0) as usize;
    let max_y = (screen_pos.y + radius_pixels)
        .ceil()
        .min(height as f32 - 1.0) as usize;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 + 0.5 - screen_pos.x;
            let dy = y as f32 + 0.5 - screen_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance > radius_pixels {
                continue;
            }

            let normalized = 1.0 - (distance / radius_pixels).powf(emitter.falloff.max(0.1));
            let weight = (normalized * emitter.intensity).clamp(0.0, 1.0);
            if weight <= 0.0 {
                continue;
            }

            let idx = y * width + x;
            buffer[idx] = blend_color(buffer[idx], emitter.color, weight);
        }
    }
}

fn apply_light_with_shadows(
    screen_pos: Vec2,
    emitter: &LightEmitter,
    occluders: &[LightOccluder],
    buffer: &mut [LinearRgba],
    width: usize,
    height: usize,
    overlay_scale: u32,
) {
    let radius_pixels = emitter.radius * overlay_scale as f32;
    if radius_pixels <= 1.0 {
        return;
    }

    let min_x = (screen_pos.x - radius_pixels).floor().max(0.0) as usize;
    let max_x = (screen_pos.x + radius_pixels)
        .ceil()
        .min(width as f32 - 1.0) as usize;
    let min_y = (screen_pos.y - radius_pixels).floor().max(0.0) as usize;
    let max_y = (screen_pos.y + radius_pixels)
        .ceil()
        .min(height as f32 - 1.0) as usize;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel_pos = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let dx = pixel_pos.x - screen_pos.x;
            let dy = pixel_pos.y - screen_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance > radius_pixels {
                continue;
            }

            let occlusion = occlusion_factor(pixel_pos, screen_pos, occluders);
            let transmit = 1.0 - occlusion * 0.9;
            if transmit <= 0.02 {
                continue;
            }

            let normalized = 1.0 - (distance / radius_pixels).powf(emitter.falloff.max(0.1));
            let weight = (normalized * emitter.intensity * transmit).clamp(0.0, 1.0);
            if weight <= 0.0 {
                continue;
            }

            let idx = y * width + x;
            buffer[idx] = blend_color(buffer[idx], emitter.color, weight);
        }
    }
}
