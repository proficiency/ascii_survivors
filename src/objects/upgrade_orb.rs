use crate::objects::{GridPosition, MoveSpeed, PlayerTag};
use crate::plugins::world::cleanup::Despawn;
use crate::resources::CameraOffset;
use crate::upgrades::{ShowUpgradeSelectionEvent, UpgradeSource};
use bevy::prelude::*;

#[derive(Component)]
pub struct UpgradeOrb {
    pub position: IVec2,
    pub precise_position: Vec2,
}

impl UpgradeOrb {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
            precise_position: position.as_vec2(),
        }
    }
}

pub fn upgrade_orb_movement(
    mut orb_query: Query<&mut UpgradeOrb>,
    player_query: Query<(&GridPosition, &MoveSpeed), With<PlayerTag>>,
    time: Res<Time>,
    _camera_offset: Res<CameraOffset>,
) {
    if let Ok((pos, speed)) = player_query.single() {
        let player_world_pos = pos.world;

        for mut orb in orb_query.iter_mut() {
            let direction_to_player = (player_world_pos - orb.position).as_vec2();
            let distance = direction_to_player.length();
            let max_speed: f32 = **speed * 1.15;

            const ATTRACTION_RADIUS: f32 = 20.0;
            const MIN_SPEED: f32 = 2.0;

            if distance > 0.0 && distance <= ATTRACTION_RADIUS {
                let speed_factor = 1.0 - (distance / ATTRACTION_RADIUS);
                let speed = MIN_SPEED + (max_speed - MIN_SPEED) * speed_factor * speed_factor;
                let movement = direction_to_player.normalize() * speed * time.delta_secs();
                orb.precise_position += movement;
                orb.position = orb.precise_position.as_ivec2();
            }
        }
    }
}

pub fn process_upgrade_orb_collection(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<PlayerTag>>,
    orb_query: Query<(Entity, &UpgradeOrb)>,
    mut upgrade_events: EventWriter<ShowUpgradeSelectionEvent>,
    _camera_offset: Res<CameraOffset>,
) {
    if let Ok(pos) = player_query.single() {
        let player_world_pos = pos.world;

        for (orb_entity, orb) in orb_query.iter() {
            let distance = (player_world_pos - orb.position).as_vec2().length();
            if distance <= 1.0 {
                commands.entity(orb_entity).insert(Despawn);
                upgrade_events.write(ShowUpgradeSelectionEvent {
                    source: UpgradeSource::BossOrb,
                });
            }
        }
    }
}
