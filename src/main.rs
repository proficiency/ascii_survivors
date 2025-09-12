mod sound;

// infinitely scaling ascii souls game
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng; // For random number generation
use crate::sound::SoundManager;



// Define a simple color enum for terminal colors
#[derive(Debug, Clone, Copy)]
enum TerminalColor {
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
}

// Implement a simple method to apply color (this is a placeholder, as bevy_ascii_terminal might have its own way)
// For now, we'll just draw characters without color

trait Upgrade {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn apply(&self, ruleset: &mut Ruleset);
}

#[derive(Debug)]
struct SpellUpgrade {

}

impl Upgrade for SpellUpgrade {
    fn name(&self) -> &str {
        "Spell Upgrade"
    }

    fn description(&self) -> &str {
        "Makes dat fireball supa hot. Mega effective"
    }

    fn apply(&self, ruleset: &mut Ruleset) {
        // eventually take a Spell struct and modify it, constrain based on the upgrade type
        ruleset.player_damage_modifier *= 1.2;
    }
}

#[derive(Debug, Default)]
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

// Player component
#[derive(Component)]
struct Player {
    health: f32,
    max_health: f32,
    speed: f32,
    position: IVec2, // Store player's position
}

// Enemy component
#[derive(Component)]
struct Enemy {
    health: f32,
    max_health: f32,
    position: IVec2, // Store enemy's position
}

impl Enemy {
    // Constructor for Enemy with default health
    fn new(position: IVec2) -> Self {
        Self {
            health: 50.0, // Two-hit kill
            max_health: 50.0,
            position,
        }
    }
}

// Projectile component
#[derive(Component)]
struct Projectile {
    position: IVec2,
    // direction: IVec2, // Direction of movement (no longer needed for homing)
    target: Option<Entity>, // The enemy this projectile is targeting
    damage: f32,
    speed: f32,
}

// Timer for enemy spawning
#[derive(Resource)]
struct EnemySpawnTimer(Timer);

// Timer for projectile cooldown
#[derive(Resource)]
struct ProjectileCooldownTimer(Timer);

// Timer for player movement
#[derive(Resource)]
struct PlayerMovementTimer(Timer);

// Timer for enemy movement
#[derive(Resource)]
struct EnemyMovementTimer(Timer);

// Offset for camera/view
#[derive(Resource)]
struct CameraOffset(IVec2);

struct GameState {
    pub ruleset: Ruleset,
    pub spawn_queue: Vec<i32>,
    pub store: Vec<i32>,
    pub sound_manager: SoundManager,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ruleset: Ruleset::default(),
            spawn_queue: Vec::new(),
            store: Vec::new(),
            sound_manager: SoundManager::new("../../assets/sfx/".into()).unwrap(), // todo: error handling
        }
    }

    pub fn constrain() {

    }

    pub fn update() {

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement, 
            update_terminal,
            spawn_enemies,
            (enemy_ai, auto_cast, move_projectiles, check_collisions).chain(), // Chain these systems to run in order
        ))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating))) // Spawn enemies every 1 second
        .insert_resource(ProjectileCooldownTimer(Timer::from_seconds(1.0, TimerMode::Once))) // Cooldown for projectiles
        .insert_resource(PlayerMovementTimer(Timer::from_seconds(0.1, TimerMode::Repeating))) // Player moves every 100 milliseconds
        .insert_resource(EnemyMovementTimer(Timer::from_seconds(0.5, TimerMode::Repeating))) // Enemies move every 500 milliseconds
        .insert_resource(CameraOffset(IVec2::new(0, 0))) // Initial camera offset
        .run();
}

fn setup(mut commands: Commands) {
    let _state = GameState::new(); // Use _state to indicate it's intentionally unused
    // state.sound_manager.play_sound("assets/sfx/45_Charge_05.wav".into(), -8.0).unwrap(); // Corrected path
    
    // Spawn the terminal
    commands.spawn((
        Terminal::new([80, 50]), // Create an 80x50 terminal
        TerminalBorder::single_line(),
    ));
    
    // Spawn player with Transform component for movement, centered
    commands.spawn((
        Player {
            health: 100.0,
            max_health: 100.0,
            speed: 1.0,
            position: IVec2::new(40, 25), // Start at center of the terminal (80/2, 50/2)
        },
        Transform::default(), // Add Transform component directly
    ));

    commands.spawn(TerminalCamera::new());
}

// System to handle player movement
fn player_movement(
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMovementTimer>,
    mut camera_offset: ResMut<CameraOffset>,
    terminal_query: Query<&Terminal>,
) {
    // Tick the timer
    timer.0.tick(time.delta());
    
    // Check if it's time to move the player
    if timer.0.finished() {
        if let Ok(mut player) = player_query.single_mut() {
            // Get the terminal size to determine the center
            if let Ok(terminal) = terminal_query.single() {
                let size = terminal.size();
                let center_x = size[0] as i32 / 2;
                let center_y = size[1] as i32 / 2;
                
                // Handle movement input (invert Y-axis for terminal coordinates)
                let mut move_offset = IVec2::new(0, 0);
                if keyboard_input.pressed(KeyCode::KeyW) {
                    move_offset.y += 1; // Move up in terminal coordinates
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    move_offset.y -= 1; // Move down in terminal coordinates
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    move_offset.x -= 1;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    move_offset.x += 1;
                }
                
                // Update camera offset instead of player position
                camera_offset.0 -= move_offset;
                
                // Keep the player's position at the center of the terminal
                player.position = IVec2::new(center_x, center_y);
            }
        }
    }
}

// System to spawn enemies from the edges of the map
fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(terminal) = terminal_query.single() { // Use single instead of get_single
        // Tick the timer
        timer.0.tick(time.delta());

        // Check if it's time to spawn a new enemy
        if timer.0.finished() {
            // Get terminal size
            let size = terminal.size();
            
            // Generate a random position on the edge of the terminal
            let mut rng = rand::thread_rng();
            let (x, y) = match rng.gen_range(0..4) {
                // Top edge
                0 => (rng.gen_range(0..size[0] as i32), 0),
                // Bottom edge
                1 => (rng.gen_range(0..size[0] as i32), size[1] as i32 - 1),
                // Left edge
                2 => (0, rng.gen_range(0..size[1] as i32)),
                // Right edge
                _ => (size[0] as i32 - 1, rng.gen_range(0..size[1] as i32)),
            };
            
            // Apply camera offset to the spawn position
            let spawn_position = IVec2::new(x, y) + camera_offset.0;
            
            // Spawn the enemy
            commands.spawn((
                Enemy::new(spawn_position),
            ));
        }
    }
}

// Basic enemy AI to move towards the player
fn enemy_ai(
    mut enemy_query: Query<&mut Enemy>,
    player_query: Query<&Player>,
    time: Res<Time>,
    mut timer: ResMut<EnemyMovementTimer>,
    camera_offset: Res<CameraOffset>,
) {
    // Tick the timer
    timer.0.tick(time.delta());
    
    // Check if it's time to move the enemies
    if timer.0.finished() {
        if let Ok(player) = player_query.single() { // Use single instead of get_single
            // Collect all current enemy positions to check for collisions
            let enemy_positions: Vec<IVec2> = enemy_query.iter().map(|enemy| enemy.position).collect();
            
            for mut enemy in enemy_query.iter_mut() {
                // Calculate direction towards the player
                let direction = player.position - (enemy.position + camera_offset.0);
                
                // Move enemy towards the player (simple step)
                if direction.x != 0 || direction.y != 0 {
                    let move_direction = direction.signum(); // Move one step in the direction of the player
                    let new_position = enemy.position + move_direction;
                    
                    // Check if the new position is occupied by another enemy
                    let mut is_occupied = false;
                    for &pos in &enemy_positions {
                        if pos == new_position {
                            is_occupied = true;
                            break;
                        }
                    }
                    
                    // Check if the new position is occupied by the player
                    if player.position == new_position {
                        is_occupied = true;
                    }
                    
                    // If the new position is not occupied, move the enemy
                    if !is_occupied {
                        enemy.position = new_position;
                    }
                }
            }
        }
    }
}

// System to automatically cast a projectile
fn auto_cast(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<(Entity, &Enemy)>, // Query for both Entity and Enemy component
    time: Res<Time>,
    mut timer: ResMut<ProjectileCooldownTimer>,
    camera_offset: Res<CameraOffset>,
) {
    // Tick the timer
    timer.0.tick(time.delta());

    // Check if it's time to cast a new projectile and if the cooldown has finished
    if timer.0.finished() {
        if let Ok(player) = player_query.single() { // Use single instead of get_single
            // Find the nearest enemy
            let mut nearest_enemy_entity: Option<Entity> = None;
            let mut min_distance = f32::MAX;
            
            for (enemy_entity, enemy) in enemy_query.iter() {
                let distance = (enemy.position + camera_offset.0 - player.position).length_squared(); // Use squared distance for efficiency
                if distance < min_distance as i32 { // Cast min_distance to i32 for comparison
                    min_distance = distance as f32; // Cast to f32 to match min_distance type
                    nearest_enemy_entity = Some(enemy_entity);
                }
            }
            
            // If there's a nearest enemy, cast a projectile towards it
            if let Some(target_entity) = nearest_enemy_entity {
                // Calculate the player's world position
                let player_world_position = player.position - camera_offset.0;
                
                // Spawn multiple projectiles in a burst (e.g., 3 projectiles)
                for _ in 0..3 {
                    commands.spawn((
                        Projectile {
                            position: player_world_position, // Spawn at player's world position
                            target: Some(target_entity), // Set the target
                            damage: 25.0, // Base damage
                            speed: 2.0, // Increase speed for a more visible effect
                        },
                    ));
                }
                
                // Reset the cooldown timer
                timer.0.reset();
            }
        }
    }
}

// System to move projectiles towards their target and despawn them if they go off-screen or the target is dead
fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    enemy_query: Query<&Enemy>,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(terminal) = terminal_query.single() {
        let terminal_size = terminal.size();
        
        for (entity, mut projectile) in projectile_query.iter_mut() {
            // Store the speed in a local variable to avoid borrowing issues
            let speed = projectile.speed;
            
            // Check if the target enemy still exists
            let mut target_exists = false;
            if let Some(target_entity) = projectile.target {
                if let Ok(target_enemy) = enemy_query.get(target_entity) {
                    target_exists = true;
                    // Calculate direction towards the target in world space
                    let direction = (target_enemy.position - projectile.position).as_vec2().normalize_or_zero();
                    
                    // Update projectile position based on its direction and speed in world space
                    // Use a slower speed for a more visible effect
                    projectile.position += (direction * speed).as_ivec2();
                }
            }
            
            // If the target is dead or there was no target, despawn the projectile
            if !target_exists {
                commands.entity(entity).despawn();
            }
            
            // Check if projectile is off-screen relative to the camera
            let draw_position = projectile.position + camera_offset.0;
            if draw_position.x < 0 || draw_position.x >= terminal_size[0] as i32 ||
               draw_position.y < 0 || draw_position.y >= terminal_size[1] as i32 {
                // Despawn the projectile
                commands.entity(entity).despawn();
            }
        }
    }
}

// System to check for collisions between projectiles and enemies
fn check_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile)>,
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    _camera_offset: Res<CameraOffset>, // Mark as unused
) {
    for (projectile_entity, projectile) in projectile_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            // Check if projectile and enemy are at the exact same position in world space
            if projectile.position == enemy.position {
                // Apply damage to the enemy
                enemy.health -= projectile.damage;
                
                // If enemy's health is depleted, despawn it
                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }
                
                // Despawn the projectile
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

// System to update the terminal with game entities
fn update_terminal(
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    projectile_query: Query<&Projectile>,
    mut terminal_query: Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() { // Use single_mut instead of get_single_mut
        // Clear the terminal
        terminal.clear();
        
        // Draw player (Cyan) - Player is always at the center
        if let Ok(player) = player_query.single() { // Use single instead of get_single
            terminal.put_char([player.position.x, player.position.y], '@');
        }
        
        // Draw enemies (Red)
        for enemy in enemy_query.iter() {
            // Apply camera offset to enemy position for drawing
            let draw_position = enemy.position + camera_offset.0;
            
            // Boundary check for enemies
            if draw_position.x >= 0 && draw_position.x < terminal.size()[0] as i32 &&
               draw_position.y >= 0 && draw_position.y < terminal.size()[1] as i32 {
                terminal.put_char([draw_position.x, draw_position.y], 'd');
            }
        }
        
        // Draw projectiles (Yellow)
        for projectile in projectile_query.iter() {
            // Apply camera offset to projectile position for drawing
            let draw_position = projectile.position + camera_offset.0;
            
            // Boundary check for projectiles
            if draw_position.x >= 0 && draw_position.x < terminal.size()[0] as i32 &&
               draw_position.y >= 0 && draw_position.y < terminal.size()[1] as i32 {
                terminal.put_char([draw_position.x, draw_position.y], '*');
            }
        }
        
        // Draw player health bar (Green)
        if let Ok(player) = player_query.single() { // Use single instead of get_single
            let health_ratio = player.health / player.max_health;
            let bar_length = 20; // Length of the health bar
            let filled_length = (health_ratio * bar_length as f32) as usize;
            
            // Create the health bar string
            let mut health_bar = String::from("[HP: ");
            for i in 0..bar_length {
                if i < filled_length {
                    health_bar.push('#');
                } else {
                    health_bar.push('-');
                }
            }
            health_bar.push(']');
            
            // Draw the health bar at a fixed position (e.g., top-left corner)
            // Make sure the health bar fits within the terminal
            if bar_length + 6 <= terminal.size()[0] as usize { // +6 for "[HP: " and "]"
                terminal.put_string([0, 0], health_bar);
            }
        }
    }
}
