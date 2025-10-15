use bevy::prelude::*;

use crate::objects::boss::Boss;
use crate::objects::player::Player;
use crate::{effects::*, resources::*};

pub fn boss_ai(
    mut commands: Commands,
    mut boss_query: Query<&mut Boss>,
    mut player_query: Query<(Entity, &mut Player)>,
    time: Res<Time>,
    mut timer: ResMut<EnemyMovementTimer>,
    mut damage_effect_timer: ResMut<DamageEffectTimer>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }

    let Ok((player_entity, mut player)) = player_query.single_mut() else {
        return;
    };

    let player_world_pos = player.world_position;
    let mut player_damage_taken = 0.0;

    for mut boss in boss_query.iter_mut() {
        let head_position = boss.get_head_position();
        let direction_to_player = (player_world_pos - head_position).as_vec2();
        let attraction_force = if direction_to_player.length() > 0.0 {
            direction_to_player.normalize() * boss.speed
        } else {
            Vec2::ZERO
        };

        let combined_force = attraction_force;
        if combined_force.length() > 0.1 {
            let normalized_direction = combined_force.normalize();
            let move_direction = IVec2::new(
                if normalized_direction.x > 0.3 {
                    1
                } else if normalized_direction.x < -0.3 {
                    -1
                } else {
                    0
                },
                if normalized_direction.y > 0.3 {
                    1
                } else if normalized_direction.y < -0.3 {
                    -1
                } else {
                    0
                },
            );

            if move_direction != IVec2::ZERO {
                let wish_move = head_position + move_direction;
                move_boss(&mut boss, wish_move);

                // if any part of the boss touches the player, deal damage
                for segment in &boss.segments {
                    if segment.position == player_world_pos {
                        player_damage_taken += boss.damage;
                        break;
                    }
                }
            }
        }
    }

    if player_damage_taken > 0.0 {
        player.health -= player_damage_taken;
        commands.entity(player_entity).insert(StatusEffect {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
        });
        damage_effect_timer.0.reset();
    }
}

fn move_boss(boss: &mut Boss, new_head_position: IVec2) {
    match boss.boss_type {
        // segments follow one another towards the head
        crate::objects::boss::BossType::Snake => {
            let mut previous_positions = vec![];
            for segment in &boss.segments {
                previous_positions.push(segment.position);
            }

            for i in (1..boss.segments.len()).rev() {
                boss.segments[i].position = boss.segments[i - 1].position;
            }

            // move the head
            if !boss.segments.is_empty() {
                boss.segments[0].position = new_head_position;
            }
        }

        // all segments move as one entity
        crate::objects::boss::BossType::Giant => {
            if !boss.segments.is_empty() {
                let old_head_position = boss.segments[0].position;
                boss.segments[0].position = new_head_position;

                // move all segments towards the new head position
                let offset = new_head_position - old_head_position;
                for i in 1..boss.segments.len() {
                    boss.segments[i].position += offset;
                }
            }
        }
    }
}
