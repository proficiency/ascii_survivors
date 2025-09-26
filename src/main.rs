mod sound;
mod systems;
use crate::sound::*;
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::*;

/*trait Upgrade {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn apply(&self, ruleset: &mut Ruleset);
}

#[derive(Debug)]
struct SpellUpgrade {}

impl Upgrade for SpellUpgrade {
    fn name(&self) -> &str {
        "Spell Upgrade"
    }

    fn description(&self) -> &str {
        "Makes dat fireball supa hot. Mega effective"
    }

    fn apply(&self, ruleset: &mut Ruleset) {
        // todo: eventually take a Spell struct and modify it, constrain based on the upgrade type
        ruleset.player_damage_modifier *= 1.2;
    }
}*/

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

#[derive(Component)]
struct Player {
    health: f32,
    max_health: f32,
    speed: f32,
    position: IVec2,
}

#[derive(Component)]
struct Enemy {
    health: f32,
    max_health: f32,
    position: IVec2,
}

impl Enemy {
    fn new(position: IVec2) -> Self {
        Self {
            health: 50.0, // two hit kill
            max_health: 50.0,
            position,
        }
    }
}

#[derive(Component)]
struct Projectile {
    position: IVec2,
    // direction: IVec2,
    target: Option<Entity>,
    damage: f32,
    speed: f32,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
struct ProjectileCooldownTimer(Timer);

#[derive(Resource)]
struct PlayerMovementTimer(Timer);

#[derive(Resource)]
struct EnemyMovementTimer(Timer);

#[derive(Resource)]
struct CameraOffset(IVec2);

struct GameState {
    pub ruleset: Ruleset,
    pub spawn_queue: Vec<i32>,
    pub store: Vec<i32>,
    pub sound_manager: SoundManager,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ((
                player_movement,
                spawn_enemies,
                (enemy_ai, auto_cast, process_projectiles, process_collisions).chain(),
                systems::render::draw_scene,
            )
                .chain(),),
        )
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        ))) // spawn enemies every second
        .insert_resource(ProjectileCooldownTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )))
        .insert_resource(PlayerMovementTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(EnemyMovementTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert_resource(CameraOffset(IVec2::default()))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([80, 50]), TerminalBorder::single_line()));
    commands.spawn((
        Player {
            health: 100.0,
            max_health: 100.0,
            speed: 1.0,
            position: IVec2::new(40, 25), // spawn the player in the center of our viewport
        },
        Transform::default(),
    ));
    commands.spawn(TerminalCamera::new());
}

fn player_movement(
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMovementTimer>,
    mut camera_offset: ResMut<CameraOffset>,
    terminal_query: Query<&Terminal>,
) {
    timer.0.tick(time.delta());
    //if timer.0.finished() {
    if let Ok(mut player) = player_query.single_mut() {
        if let Ok(terminal) = terminal_query.single() {
            let size = terminal.size();
            let center_x = size[0] as i32 / 2;
            let center_y = size[1] as i32 / 2;

            let mut move_offset = IVec2::new(0, 0);
            if keyboard_input.pressed(KeyCode::KeyW) {
                move_offset.y += 1;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                move_offset.y -= 1;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                move_offset.x -= 1;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                move_offset.x += 1;
            }

            // todo: this is kinda weird
            camera_offset.0 -= move_offset;
            player.position = IVec2::new(center_x, center_y);
        }
    }
    //}
}

fn spawn_enemies(
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

fn enemy_ai(
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
                let direction = player.position - (enemy.position + camera_offset.0);

                // simple step towards the player
                if direction.x != 0 || direction.y != 0 {
                    let move_direction = direction.signum();
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
                    if player.position == wish_move {
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

fn auto_cast(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<(Entity, &Enemy)>,
    time: Res<Time>,
    mut timer: ResMut<ProjectileCooldownTimer>,
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
                commands.entity(entity).despawn();
            }

            // check if projectile is off-screen relative to the camera
            let draw_position = projectile.position + camera_offset.0;
            if draw_position.x < 0
                || draw_position.x >= terminal_size[0] as i32
                || draw_position.y < 0
                || draw_position.y >= terminal_size[1] as i32
            {
                // despawn
                commands.entity(entity).despawn();
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
    let mut despawned_entities: Vec<Entity> = Vec::new();

    for (projectile_entity, projectile) in projectile_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            if projectile.position == enemy.position {
                // take damage
                enemy.health -= projectile.damage;

                // if enemy's health pool is depleted, despawn it
                if enemy.health <= 0.0 && !despawned_entities.contains(&enemy_entity) {
                    if commands.get_entity(enemy_entity).is_ok() {
                        commands.entity(enemy_entity).despawn();
                        despawned_entities.push(enemy_entity);
                    }
                }

                if commands.get_entity(projectile_entity).is_ok()
                    && !despawned_entities.contains(&projectile_entity)
                {
                    commands.entity(projectile_entity).despawn();
                    despawned_entities.push(projectile_entity);
                }
            }
        }
    }
}