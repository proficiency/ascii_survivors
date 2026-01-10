use crate::{maps::Map, objects::*, resources::*};
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;

/// margin to ensure the portal spawns away from the edges of the screen
const PORTAL_SPAWN_MARGIN: i32 = 5;

/// radius around the player where the portal should never spawn
const PLAYER_SAFE_RADIUS: i32 = 3;
const MAX_PORTAL_SPAWN_ATTEMPTS: usize = 64;

#[allow(clippy::too_many_arguments)]
pub fn spawn_portal_after_survival(
    mut commands: Commands,
    survival_timer: Res<SurvivalTimer>,
    ruleset: Res<Ruleset>,
    player_query: Query<&Player>,
    portal_query: Query<&Portal>,
    grid: Res<AsciiGrid>,
    camera_offset: Res<CameraOffset>,
    level: Res<Level>,
    map: Option<Res<Map>>,
) {
    if level.as_ref() == &Level::Rest {
        return;
    }

    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time && portal_query.is_empty() {
        let width = grid.grid_size.x as i32;
        let height = grid.grid_size.y as i32;
        let min_x = camera_offset.0.x + PORTAL_SPAWN_MARGIN;
        let max_x = min_x + width - 1 - 2 * PORTAL_SPAWN_MARGIN;
        let min_y = camera_offset.0.y + PORTAL_SPAWN_MARGIN;
        let max_y = min_y + height - 1 - 2 * PORTAL_SPAWN_MARGIN;

        if min_x <= max_x
            && min_y <= max_y
            && let Ok(player) = player_query.single()
            && let Some(map) = map.as_deref()
        {
            let player_world_position = player.world_position;

            // generate a random position within the visible area
            // that is not too close to the player
            let mut rng = rand::rng();
            let mut portal_position = None;

            for _ in 0..MAX_PORTAL_SPAWN_ATTEMPTS {
                let portal_x = rng.random_range(min_x..=max_x);
                let portal_y = rng.random_range(min_y..=max_y);
                let candidate = IVec2::new(portal_x, portal_y);

                let distance = (candidate - player_world_position).length_squared();
                if distance <= PLAYER_SAFE_RADIUS * PLAYER_SAFE_RADIUS {
                    continue;
                }

                if map.is_walkable(candidate.x, candidate.y)
                    && has_walkable_path(map, candidate, player_world_position)
                {
                    portal_position = Some(candidate);
                    break;
                }
            }

            if let Some(portal_position) = portal_position {
                commands.spawn((Portal::new(portal_position),));
            }
        }
    }
}

fn has_walkable_path(map: &Map, start: IVec2, goal: IVec2) -> bool {
    if start == goal {
        return true;
    }

    if !map.is_walkable(goal.x, goal.y) {
        return false;
    }

    let width = map.width;
    let height = map.height;
    let mut visited = vec![false; width * height];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    mark_visited(&mut visited, width, start);

    while let Some(current) = queue.pop_front() {
        for neighbor in neighbors(current) {
            if !map.in_bounds(neighbor.x, neighbor.y) {
                continue;
            }

            if !map.is_walkable(neighbor.x, neighbor.y) {
                continue;
            }

            if was_visited(&visited, width, neighbor) {
                continue;
            }

            if neighbor == goal {
                return true;
            }

            mark_visited(&mut visited, width, neighbor);
            queue.push_back(neighbor);
        }
    }

    false
}

fn neighbors(position: IVec2) -> [IVec2; 4] {
    [
        IVec2::new(position.x + 1, position.y),
        IVec2::new(position.x - 1, position.y),
        IVec2::new(position.x, position.y + 1),
        IVec2::new(position.x, position.y - 1),
    ]
}

fn index_for(width: usize, position: IVec2) -> usize {
    position.y as usize * width + position.x as usize
}

fn was_visited(visited: &[bool], width: usize, position: IVec2) -> bool {
    visited[index_for(width, position)]
}

fn mark_visited(visited: &mut [bool], width: usize, position: IVec2) {
    let index = index_for(width, position);
    visited[index] = true;
}
