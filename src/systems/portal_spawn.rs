use crate::{maps::Map, objects::*, resources::*};
use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;
use rand::prelude::*;

/// margin to ensure the portal spawns away from the edges of the screen
const PORTAL_SPAWN_MARGIN: i32 = 5;

/// radius around the player where the portal should never spawn
const PLAYER_SAFE_RADIUS: i32 = 3;

pub fn spawn_portal_after_survival(
    mut commands: Commands,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    player_query: Query<&Player>,
    portal_query: Query<&Portal>,
    terminal_query: Query<&Terminal>,
    mut scene_lock: ResMut<SceneLock>,
    _camera_offset: Res<CameraOffset>,
    level: Res<Level>,
    map: Option<Res<Map>>,
) {
    if level.as_ref() == &Level::Rest {
        return;
    }

    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time {
        if portal_query.is_empty() {
            scene_lock.0 = true;

            if let (Ok(_terminal), Some(map)) = (terminal_query.single(), map.as_ref()) {
                let bounds_min = IVec2::splat(PORTAL_SPAWN_MARGIN);
                let bounds_max = IVec2::new(
                    map.width as i32 - 1 - PORTAL_SPAWN_MARGIN,
                    map.height as i32 - 1 - PORTAL_SPAWN_MARGIN,
                );

                if bounds_min.x <= bounds_max.x && bounds_min.y <= bounds_max.y {
                    if let Ok(player) = player_query.single() {
                        let player_position = player.world_position;

                        // generate a random position within the map bounds
                        // that is walkable and not too close to the player
                        let mut rng = rand::rng();
                        let mut attempts = 0;
                        let portal_position = loop {
                            let portal_x = rng.random_range(bounds_min.x..=bounds_max.x);
                            let portal_y = rng.random_range(bounds_min.y..=bounds_max.y);
                            let portal_position = IVec2::new(portal_x, portal_y);

                            let distance = (portal_position - player_position).length_squared();
                            let far_enough = distance > PLAYER_SAFE_RADIUS * PLAYER_SAFE_RADIUS;
                            let walkable = map.is_walkable(portal_position.x, portal_position.y);

                            if far_enough && walkable {
                                break portal_position;
                            }

                            attempts += 1;
                            if attempts > 512 {
                                // fallback to player-safe constraint only to avoid rare infinite loops
                                break portal_position;
                            }
                        };

                        commands.spawn((Portal::new(portal_position),));
                    }
                }
            }
        }
    }
}
