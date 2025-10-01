use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;
use rand::Rng;

use crate::{
    objects::portal::Portal, 
    resources::{timers::SurvivalTimer, scene_lock::SceneLock, camera::CameraOffset, ruleset::Ruleset}, 
    Player
};

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
    camera_offset: Res<CameraOffset>,
) {
    if survival_timer.0.elapsed_secs() >= ruleset.portal_spawn_time {
        if portal_query.is_empty() {
            scene_lock.0 = true;
            
            if let Ok(terminal) = terminal_query.single() {
                let terminal_size = terminal.size();
                let width = terminal_size[0] as i32;
                let height = terminal_size[1] as i32;
                
                let min_x = -camera_offset.0.x + PORTAL_SPAWN_MARGIN;
                let max_x = min_x + width - 1 - 2 * PORTAL_SPAWN_MARGIN;
                let min_y = -camera_offset.0.y + PORTAL_SPAWN_MARGIN;
                let max_y = min_y + height - 1 - 2 * PORTAL_SPAWN_MARGIN;
                
                if min_x <= max_x && min_y <= max_y {
                    if let Ok(player) = player_query.single() {
                        let player_position = player.position;
                        
                        // generate a random position within the visible area
                        // that is not too close to the player
                        let mut rng = rand::thread_rng();
                        let mut portal_x;
                        let mut portal_y;
                        let mut portal_position;
                        
                        loop {
                            portal_x = rng.gen_range(min_x..=max_x);
                            portal_y = rng.gen_range(min_y..=max_y);
                            portal_position = IVec2::new(portal_x, portal_y);
                            
                            let distance = (portal_position - player_position).length_squared();
                            if distance > PLAYER_SAFE_RADIUS * PLAYER_SAFE_RADIUS {
                                break;
                            }
                        }
                        
                        commands.spawn((Portal::new(portal_position),));
                    }
                }
            }
        }
    }
}
