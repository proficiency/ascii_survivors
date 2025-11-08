use crate::objects::boss::Boss;
use crate::objects::enemy::Enemy;
use crate::objects::orb::Orb;
use crate::objects::player::Player;
use crate::resources::camera::CameraOffset;
use crate::resources::channels::*;
use crate::resources::kill_count::KillCount;
use crate::resources::scene_lock::SceneLock;
use crate::resources::timers::ProjectileCooldownTimer;
use crate::systems::cleanup::Despawn;
use crate::CameraOffset;
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub position: IVec2,
    pub target: Option<Entity>,
    pub target_last_position: Option<IVec2>,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Fireball;

pub fn auto_cast(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<(Entity, &Enemy)>,
    boss_query: Query<(Entity, &Boss)>,
    time: Res<Time>,
    mut timer: ResMut<ProjectileCooldownTimer>,
    audio: Res<AudioChannel<Sfx>>,
    asset_server: Res<AssetServer>,
    _scene_lock: Res<SceneLock>,
) {
    timer.0.tick(time.delta());

    // is it time to fire a new projectile?
    if timer.0.finished()
        && let Ok(player) = player_query.single()
    {
        let mut nearest_target_entity: Option<Entity> = None;
        let mut min_distance = i32::MAX;

        for (enemy_entity, enemy) in enemy_query.iter() {
            let enemy_world_pos = enemy.position;
            let player_world_pos = player.world_position;

            let distance = (enemy_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(enemy_entity);
            }
        }

        for (boss_entity, boss) in boss_query.iter() {
            let boss_world_pos = boss.get_head_position();
            let player_world_pos = player.world_position;

            let distance = (boss_world_pos - player_world_pos).length_squared();
            if distance < min_distance {
                min_distance = distance;
                nearest_target_entity = Some(boss_entity);
            }
        }

        // if we're targeting the nearest enemy, attack it
        if let Some(target_entity) = nearest_target_entity {
            let player_position = player.world_position;

            for _ in 0..3 {
                commands.spawn((Projectile {
                    position: player_position,   // spawn at player origin
                    target: Some(target_entity), // travel towards a target
                    target_last_position: None,  // no last position yet
                    damage: 25.0,                // do some damage
                    speed: 1.65,                 // travel slowly
                },));
            }

            audio
                .play(asset_server.load("sfx/25_Wind_01.wav"))
                .with_volume(0.25);
            timer.0.reset();
        }
    }
}

pub fn process_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    enemy_query: Query<&Enemy>,
    boss_query: Query<&Boss>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
    _scene_lock: Res<SceneLock>,
) {
    if let Ok(terminal) = terminal_query.single() {
        let terminal_size = terminal.size();

        for (entity, mut projectile) in projectile_query.iter_mut() {
            let speed = projectile.speed;
            let mut target_exists = false;

            if let Some(target_entity) = projectile.target {
                if let Ok(target_enemy) = enemy_query.get(target_entity) {
                    projectile.target_last_position = Some(target_enemy.position);
                    
                    let direction = (target_enemy.position - projectile.position)
                        .as_vec2()
                        .normalize_or_zero();

                    target_exists = true;
                    projectile.position += (direction * speed).as_ivec2();
                }
              
                // we can't find an enemy, but are there any bosses?
                else if let Ok(target_boss) = boss_query.get(target_entity) {
                    projectile.target_last_position = Some(target_boss.get_head_position());
                    
                    let direction = (target_boss.get_head_position() - projectile.position)
                        .as_vec2()
                        .normalize_or_zero();

                    target_exists = true;
                    projectile.position += (direction * speed).as_ivec2();
                }
                else if projectile.target_last_position.is_some() {
                    let last_position = projectile.target_last_position.unwrap();
                    let direction = (last_position - projectile.position)
                        .as_vec2()
                        .normalize_or_zero();
                        
                    projectile.position += (direction * speed).as_ivec2();
                    
                    if projectile.position == last_position {
                        target_exists = false;
                    } else {
                        target_exists = true;
                    }
                }
            }

            // ensure the projectile is despawned if the target is dead or there was no valid target
            // and we've reached the last known position
            if !target_exists {
                commands.entity(entity).insert(Despawn);
            }

            // check if projectile is off-screen relative to the camera
            let draw_position = projectile.position + camera_offset.0;
            if draw_position.x < 0
                || draw_position.x >= terminal_size[0] as i32
                || draw_position.y < 0
                || draw_position.y >= terminal_size[1] as i32
            {
                // despawn
                commands.entity(entity).insert(Despawn);
            }
        }
    }
}

pub fn process_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile)>,
    mut enemy_query: Query<(Entity, &mut Enemy), Without<Despawn>>,
    mut boss_query: Query<(Entity, &mut Boss), Without<Despawn>>,
    mut kill_count: ResMut<KillCount>,
    _scene_lock: Res<SceneLock>,
) {
    // todo: currently only checking projectiles against enemies
    for (projectile_entity, projectile) in projectile_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            if projectile.position == enemy.position {
                if enemy.health > 0.0 {
                    // take damage
                    enemy.health -= projectile.damage;

                    // if enemy's health pool is depleted, mark it for despawn
                    if enemy.health <= 0.0 {
                        // spawn an orb at the enemy's position before despawning
                        commands.spawn(Orb::new(enemy.position, 10));
                        commands.entity(enemy_entity).insert(Despawn);
                        kill_count.enemies += 1;
                    }
                }

                // mark projectile for despawn
                commands.entity(projectile_entity).insert(Despawn);
            }
        }

        for (boss_entity, mut boss) in boss_query.iter_mut() {
            for (segment_index, segment) in boss.segments.iter().enumerate() {
                if projectile.position == segment.position {
                    let is_defeated = boss.take_damage(projectile.damage, segment_index);
                    if is_defeated {
                        for segment in &boss.segments {
                            commands.spawn(Orb::new(segment.position, 50)); // bosses are worth more experience than normal enemies
                        }
                        commands.entity(boss_entity).insert(Despawn);
                        kill_count.enemies += 1;
                    }

                    commands.entity(projectile_entity).insert(Despawn);
                    break; // ensure a projectile can only damage one segment at a time
                }
            }
        }
    }
}
