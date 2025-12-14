use crate::{maps::*, objects::*, plugins::input::WalkEvent, resources::*};
use bevy::prelude::*;
use bevy_ascii_terminal::*;

pub fn player_movement(
    mut player_query: Query<&mut Player>,
    mut walk_events: EventReader<WalkEvent>,
    mut camera_offset: ResMut<CameraOffset>,
    terminal_query: Query<&Terminal>,
    scene_lock: Res<SceneLock>,
    map: Option<Res<Map>>,
) {
    if let Ok(mut player) = player_query.single_mut()
        && let Ok(terminal) = terminal_query.single()
        && let Some(map) = map.as_deref()
    {
        let size = terminal.size();
        let center_x = size[0] as i32 / 2;
        let center_y = size[1] as i32 / 2;

        for WalkEvent { direction } in walk_events.read() {
            // should already be clamped but this is cheap
            let clamped_direction = direction.clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
            if clamped_direction == IVec2::ZERO {
                continue;
            }

            // convert direction to world delta
            let world_delta = IVec2::new(clamped_direction.x, -clamped_direction.y);

            let (position, world_position, update_camera_offset) = if scene_lock.0 {
                let new_pos = player.position + clamped_direction;

                // desired world position to occupy
                let wish_move =
                    IVec2::new(new_pos.x, size[1] as i32 - 1 - new_pos.y) + camera_offset.0;

                (new_pos, wish_move, false)
            } else {
                // here the player remains centered, and the world around them moves
                let center = IVec2::new(center_x, center_y);
                let wish_move = player.world_position + world_delta;

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
