use bevy::prelude::*;

use crate::effects::status_effect::StatusEffect;
use crate::objects::enemy::Enemy;
use crate::objects::player::Player;
use crate::resources::scene_lock::SceneLock;
use crate::resources::timers::DamageEffectTimer;
use crate::resources::timers::EnemyMovementTimer;

pub fn enemy_ai(
    mut commands: Commands,
    mut enemy_query: Query<&mut Enemy>,
    mut player_query: Query<(Entity, &mut Player)>,
    time: Res<Time>,
    mut timer: ResMut<EnemyMovementTimer>,
    mut damage_effect_timer: ResMut<DamageEffectTimer>,
    _scene_lock: Res<SceneLock>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }

    let Ok((player_entity, mut player)) = player_query.single_mut() else {
        return;
    };

    let player_world_pos = player.world_position;
    let enemy_positions: Vec<IVec2> = enemy_query.iter().map(|enemy| enemy.position).collect();
    let mut player_damage_taken = 0.0;

    for mut enemy in enemy_query.iter_mut() {
        let direction_to_player = (player_world_pos - enemy.position).as_vec2();
        let attraction_force = if direction_to_player.length() > 0.0 {
            direction_to_player.normalize() * enemy.speed
        } else {
            Vec2::ZERO
        };

        let mut separation_force = Vec2::ZERO;
        const SEPARATION_RADIUS: f32 = 2.0;
        const SEPARATION_STRENGTH: f32 = 1.0;

        for &other_pos in &enemy_positions {
            if other_pos != enemy.position {
                let distance_vec = (enemy.position - other_pos).as_vec2();
                let distance = distance_vec.length();

                if distance < SEPARATION_RADIUS && distance > 0.0 {
                    let repulsion_strength =
                        SEPARATION_STRENGTH * (SEPARATION_RADIUS - distance) / SEPARATION_RADIUS;
                    separation_force += distance_vec.normalize() * repulsion_strength;
                }
            }
        }

        let combined_force = attraction_force + separation_force;
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
                let wish_move = enemy.position + move_direction;

                // check if the desired position is occupied by another enemy
                let mut is_occupied = false;
                for &pos in &enemy_positions {
                    if pos == wish_move {
                        is_occupied = true;
                        break;
                    }
                }

                // check if the desired position is occupied by the player
                if player_world_pos == wish_move {
                    is_occupied = true;
                    player_damage_taken += enemy.damage;
                }

                // if the desired position is not occupied, move the enemy
                if !is_occupied {
                    enemy.position = wish_move;
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
