use crate::{objects::*, resources::*};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn portal_transition_system(
    time: Res<Time>,
    mut portal_transition: ResMut<PortalTransition>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level: ResMut<Level>,
    player_query: Query<&Player>,
    portal_query: Query<&Portal>,
    audio: Res<AudioChannel<Music>>
) {
    if let Ok(player) = player_query.single() {
        let mut player_near_portal = false;
        for portal in portal_query.iter() {
            let diff = player.world_position - portal.position;
            let distance_squared = diff.x * diff.x + diff.y * diff.y;
            if distance_squared <= 1 {
                player_near_portal = true;
                break;
            }
        }

        match portal_transition.state {
            PortalTransitionState::Inactive => {
                if player_near_portal {
                    portal_transition.state = PortalTransitionState::BuildingUp;
                    portal_transition.timer.reset();
                    portal_transition.progress = 0.0;
                }
            }
            PortalTransitionState::BuildingUp => {
                if player_near_portal {
                    portal_transition.timer.tick(time.delta());
                    portal_transition.progress = portal_transition.timer.fraction();

                    if portal_transition.timer.finished() {
                        let transitioning_to_rest = matches!(level.as_ref(), Level::Survival);

                        *level = match level.as_ref() {
                            Level::Survival => Level::Rest,
                            Level::Rest => Level::Grassland,
                            Level::Grassland => Level::Dungeon,
                            Level::Dungeon => Level::Grassland,
                        };

                        if transitioning_to_rest {
                            audio.stop();
                        }

                        next_state.set(GameState::LevelTransition);
                        portal_transition.state = PortalTransitionState::Inactive;
                        portal_transition.progress = 0.0;
                        portal_transition.timer.reset();
                    }
                } else {
                    portal_transition.state = PortalTransitionState::BreakingDown;
                    let breakdown_time = portal_transition.progress * 2.0;
                    portal_transition.timer = Timer::from_seconds(breakdown_time, TimerMode::Once);
                    portal_transition.timer.reset();
                }
            }
            PortalTransitionState::BreakingDown => {
                if player_near_portal {
                    portal_transition.state = PortalTransitionState::BuildingUp;
                    let remaining_time = portal_transition.progress * 2.0;
                    portal_transition.timer = Timer::from_seconds(2.0, TimerMode::Once);
                    portal_transition
                        .timer
                        .set_elapsed(std::time::Duration::from_secs_f32(2.0 - remaining_time));
                } else {
                    portal_transition.timer.tick(time.delta());
                    portal_transition.progress = 1.0 - portal_transition.timer.fraction();

                    if portal_transition.timer.finished() {
                        portal_transition.state = PortalTransitionState::Inactive;
                        portal_transition.progress = 0.0;
                        portal_transition.timer.reset();
                    }
                }
            }
        }
    }
}

pub fn render_portal_transition(
    mut query: Query<&mut bevy_ascii_terminal::Terminal>,
    portal_transition: Res<PortalTransition>,
    player_query: Query<&Player>,
    camera_offset: Res<crate::resources::CameraOffset>,
) {
    if portal_transition.progress <= 0.0 {
        return;
    }

    if let (Ok(mut terminal), Ok(player)) = (query.single_mut(), player_query.single()) {
        let screen_pos = player.world_position + camera_offset.0;

        if screen_pos.x >= 0 && screen_pos.x < 80 && screen_pos.y >= 0 && screen_pos.y < 50 {
            let radius = (portal_transition.progress * 3.0) as i32;

            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();
                    if distance <= radius as f32 && distance >= (radius - 1) as f32 {
                        let x = screen_pos.x + dx;
                        let y = screen_pos.y + dy;
                        if x >= 0 && x < 80 && y >= 0 && y < 50 {
                            let char = match portal_transition.progress {
                                p if p < 0.25 => '░',
                                p if p < 0.5 => '▒',
                                p if p < 0.75 => '▓',
                                _ => '█',
                            };
                            terminal.put_char([x as usize, y as usize], char);
                        }
                    }
                }
            }
        }
    }
}
