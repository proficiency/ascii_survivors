mod resources;
mod systems;
mod objects;

use crate::systems::cleanup::*;
use crate::objects::orb::*;
use crate::objects::player::*;
use crate::objects::enemy::*;
use crate::objects::projectile::*;

use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::*;
use resources::sound::SoundManager;
use std::path::*;

#[derive(Resource, Debug, Default)]
struct Ruleset {
    pub enemy_health_modifier: f32,
    pub enemy_damage_modifier: f32,
    pub enemy_spawn_rate: f32,
    pub player_health_modifier: f32,
    pub player_damage_modifier: f32,
    pub player_speed_modifier: f32,

    pub enemies_slow_on_attack: bool,
    pub enemies_stun_on_attack: bool,
}

impl Ruleset {
    pub fn default() -> Self {
        Self {
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            enemy_spawn_rate: 1.0,
            player_health_modifier: 1.0,
            player_damage_modifier: 1.0,
            player_speed_modifier: 1.0,
            enemies_slow_on_attack: false,
            enemies_stun_on_attack: false,
        }
    }
}

#[derive(Resource)]
pub struct CameraOffset(pub IVec2);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, (setup, list_gamepads, play_theme).chain())
        .add_systems(
            Update,
            ((
                player_movement,
                spawn_enemies,
                (enemy_ai, auto_cast, process_projectiles, process_collisions, orb_movement, process_orb_collection).chain(),
                systems::render::draw_scene,
                despawn_entities,
            )
                .chain(),),
        )
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.25,
            TimerMode::Repeating,
        )))
        .insert_resource(ProjectileCooldownTimer(Timer::from_seconds(
            2.0,
            TimerMode::Once,
        )))
        .insert_resource(PlayerMovementTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(EnemyMovementTimer(Timer::from_seconds(
            0.35,
            TimerMode::Repeating,
        )))
        .insert_resource(CameraOffset(IVec2::default()))
        .insert_resource(SoundManager::new(PathBuf::from("./assets/sfx/")).unwrap())
        .run();
}

fn play_theme(mut sound_manager: ResMut<SoundManager>) {
    sound_manager.play_theme(-17.0).unwrap();
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([80, 50]), TerminalBorder::single_line()));
    commands.spawn((
        Player::new(IVec2::new(40, 25)),
        Transform::default(),
    ));
    commands.spawn(TerminalCamera::new());
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    println!("Looking for gamepads...");
    for (name, gamepad) in &gamepads {
        println!("Found gamepad: {name}");
    }
}

fn auto_cast(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<(Entity, &Enemy)>,
    time: Res<Time>,
    mut timer: ResMut<ProjectileCooldownTimer>,
    mut sound_manager: ResMut<SoundManager>,
    camera_offset: Res<CameraOffset>,
) {
    timer.0.tick(time.delta());

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

fn process_projectiles(
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

fn process_collisions(
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
