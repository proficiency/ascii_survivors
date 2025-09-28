use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

use crate::objects::player::Player;
use crate::CameraOffset;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub max_health: f32,
    pub position: IVec2,
}

impl Enemy {
    pub fn new(position: IVec2) -> Self {
        Self {
            health: 50.0, // two hit kill
            max_health: 50.0,
            position,
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyMovementTimer(pub Timer);

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(terminal) = terminal_query.single() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let size = terminal.size();
            let mut rng = rand::thread_rng();

            // choose a random edge to spawn the enemy at
            let (x, y) = match rng.gen_range(0..4) {
                // top edge
                0 => (rng.gen_range(0..size[0] as i32), 0),
                // bottom edge
                1 => (rng.gen_range(0..size[0] as i32), size[1] as i32 - 1),
                // left edge
                2 => (0, rng.gen_range(0..size[1] as i32)),
                // right edge
                _ => (size[0] as i32 - 1, rng.gen_range(0..size[1] as i32)),
            };

            // spawn the enemy offscreen
            let spawn_position = IVec2::new(x, y) + camera_offset.0;
            commands.spawn((Enemy::new(spawn_position),));
        }
    }
}

pub fn enemy_ai(
    mut enemy_query: Query<&mut Enemy>,
    player_query: Query<&Player>,
    time: Res<Time>,
    mut timer: ResMut<EnemyMovementTimer>,
    camera_offset: Res<CameraOffset>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        if let Ok(player) = player_query.single() {
            let enemy_positions: Vec<IVec2> =
                enemy_query.iter().map(|enemy| enemy.position).collect();

            for mut enemy in enemy_query.iter_mut() {
                let player_world_pos = player.position - camera_offset.0;
                let direction_to_player = (player_world_pos - enemy.position).as_vec2();
                let attraction_force = if direction_to_player.length() > 0.0 {
                    direction_to_player.normalize() * 1.0
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
                            let repulsion_strength = SEPARATION_STRENGTH * (SEPARATION_RADIUS - distance) / SEPARATION_RADIUS;
                            separation_force += distance_vec.normalize() * repulsion_strength;
                        }
                    }
                }

                let combined_force = attraction_force + separation_force;
                if combined_force.length() > 0.1 {
                    let normalized_direction = combined_force.normalize();
                    let move_direction = IVec2::new(
                        if normalized_direction.x > 0.3 { 1 } else if normalized_direction.x < -0.3 { -1 } else { 0 },
                        if normalized_direction.y > 0.3 { 1 } else if normalized_direction.y < -0.3 { -1 } else { 0 }
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
                        }

                        // if the desired position is not occupied, move the enemy
                        if !is_occupied {
                            enemy.position = wish_move;
                        }
                    }
                }
            }
        }
    }
}
