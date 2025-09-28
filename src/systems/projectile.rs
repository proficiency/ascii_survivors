use bevy::prelude::*;
use bevy_ascii_terminal::*;
use crate::systems::player::Player;
use crate::systems::enemy::Enemy;
use crate::systems::orbs::Orb;
use crate::systems::cleanup::Despawn;
use crate::resources::sound::SoundManager;
use crate::CameraOffset; // Import CameraOffset from main.rs

#[derive(Component)]
pub struct Projectile {
    pub position: IVec2,
    pub target: Option<Entity>,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Resource)]
pub struct ProjectileCooldownTimer(pub Timer);

pub fn auto_cast(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<(Entity, &Enemy)>,
    time: Res<Time>,
    mut timer: ResMut<ProjectileCooldownTimer>,
    mut sound_manager: ResMut<SoundManager>,
    camera_offset: Res<CameraOffset>,
) {
    timer.0.tick(time.delta());

    // is it time to fire a new projectile?
    if timer.0.finished() {
        if let Ok(player) = player_query.single() {
            let mut nearest_enemy_entity: Option<Entity> = None;
            let mut min_distance = i32::MAX;

            for (enemy_entity, enemy) in enemy_query.iter() {
                let distance =
                    (enemy.position + camera_offset.0 - player.position).length_squared();
                if distance < min_distance {
                    min_distance = distance;
                    nearest_enemy_entity = Some(enemy_entity);
                }
            }

            // if we're targeting the nearest enemy, attack it
            if let Some(target_entity) = nearest_enemy_entity {
                let player_position = player.position - camera_offset.0;

                for _ in 0..3 {
                    commands.spawn((Projectile {
                        position: player_position,   // spawn at player origin
                        target: Some(target_entity), // travel towards a target
                        damage: 25.0,                // do some damage
                        speed: 1.65,                 // travel slowly
                    },));
                }

                sound_manager
                    .play_sound("./assets/sfx/25_Wind_01.wav".into(), -30.0)
                    .ok();
                timer.0.reset();
            }
        }
    }
}

pub fn process_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    enemy_query: Query<&Enemy>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(terminal) = terminal_query.single() {
        let terminal_size = terminal.size();

        for (entity, mut projectile) in projectile_query.iter_mut() {
            let speed = projectile.speed;
            let mut target_exists = false;

            // try to find a target
            if let Some(target_entity) = projectile.target {
                if let Ok(target_enemy) = enemy_query.get(target_entity) {
                    let direction = (target_enemy.position - projectile.position)
                        .as_vec2()
                        .normalize_or_zero();

                    target_exists = true;
                    projectile.position += (direction * speed).as_ivec2();
                }
            }

            // ensure the projectile is despawned if the target is dead or there was no valid target
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
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    _camera_offset: Res<CameraOffset>, // todo.
) {
    // todo: currently only checking projectiles against enemies
    for (projectile_entity, projectile) in projectile_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            if projectile.position == enemy.position {
                // take damage
                enemy.health -= projectile.damage;

                // if enemy's health pool is depleted, mark it for despawn
                if enemy.health <= 0.0 {
                    // spawn an orb at the enemy's position before despawning
                    commands.spawn(Orb::new(enemy.position, 10));
                    commands.entity(enemy_entity).insert(Despawn);
                }

                // mark projectile for despawn
                commands.entity(projectile_entity).insert(Despawn);
            }
        }
    }
}
