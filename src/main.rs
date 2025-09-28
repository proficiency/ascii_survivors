mod objects;
mod resources;
mod systems;

use crate::objects::enemy::*;
use crate::objects::orb::*;
use crate::objects::player::*;
use crate::objects::projectile::*;
use crate::systems::cleanup::*;

use bevy::prelude::*;
use bevy_ascii_terminal::*;
use resources::sound::SoundManager;
use std::path::*;
use resources::camera::CameraOffset;
use resources::timers::{EnemySpawnTimer, ProjectileCooldownTimer, PlayerMovementTimer, EnemyMovementTimer};
use systems::player_movement::player_movement;
use systems::enemy_spawn::spawn_enemies;
use systems::enemy_ai::enemy_ai;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, (setup, list_gamepads, play_theme).chain())
        .add_systems(
            Update,
            ((
                player_movement,
                spawn_enemies,
                (
                    enemy_ai,
                    auto_cast,
                    process_projectiles,
                    process_collisions,
                    orb_movement,
                    process_orb_collection,
                )
                    .chain(),
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
    commands.spawn((Player::new(IVec2::new(40, 25)), Transform::default()));
    commands.spawn(TerminalCamera::new());
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    println!("Looking for gamepads...");
    for (name, _gamepad) in &gamepads {
        println!("Found gamepad: {name}");
    }
}
