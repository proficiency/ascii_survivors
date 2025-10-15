use crate::{objects::*, systems::Despawn, resources::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct Orb {
    pub position: IVec2,
    pub precise_position: Vec2,
    pub experience: u32,
}

impl Orb {
    pub fn new(position: IVec2, experience: u32) -> Self {
        Self {
            position,
            precise_position: position.as_vec2(),
            experience,
        }
    }
}

/// orbs within a certain radius will move towards the player with increasing speed.
pub fn orb_movement(
    mut orb_query: Query<&mut Orb>,
    player_query: Query<&Player>,
    time: Res<Time>,
    _camera_offset: Res<CameraOffset>,
) {
    if let Ok(player) = player_query.single() {
        let player_world_pos = player.world_position;

        for mut orb in orb_query.iter_mut() {
            let direction_to_player = (player_world_pos - orb.position).as_vec2();
            let distance = direction_to_player.length();
            let max_speed: f32 = player.speed * 1.15; // just so the player isn't able to outrun the orbs and create a mess

            const ATTRACTION_RADIUS: f32 = 20.0;
            const MIN_SPEED: f32 = 2.0;

            if distance > 0.0 && distance <= ATTRACTION_RADIUS {
                // speed increases as distance decreases
                let speed_factor = 1.0 - (distance / ATTRACTION_RADIUS);
                let speed = MIN_SPEED + (max_speed - MIN_SPEED) * speed_factor * speed_factor;
                let movement = direction_to_player.normalize() * speed * time.delta_secs();
                orb.precise_position += movement;
                orb.position = orb.precise_position.as_ivec2();
            }
        }
    }
}

/// when an orb is within 1 unit of the player, it is despawned and the player gains experience.
pub fn process_orb_collection(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    orb_query: Query<(Entity, &Orb)>,
    _camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        let player_world_pos = player.world_position;

        for (orb_entity, orb) in orb_query.iter() {
            let distance = (player_world_pos - orb.position).as_vec2().length();
            if distance <= 1.0 {
                player.experience += orb.experience;
                commands.entity(orb_entity).insert(Despawn);

                // Check for level up
                while player.experience >= player.experience_to_next_level {
                    player.experience -= player.experience_to_next_level;
                    player.level += 1;
                    player.experience_to_next_level = experience_for_level(player.level);
                }
            }
        }
    }
}
