mod effects;
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
use effects::damage_effect::update_damage_effect;
use resources::camera::CameraOffset;
use resources::sound::SoundManager;
use resources::timers::{
    DamageEffectTimer, EnemyMovementTimer, EnemySpawnTimer, PlayerMovementTimer,
    ProjectileCooldownTimer,
};
use std::path::*;
use systems::enemy_ai::enemy_ai;
use systems::enemy_spawn::spawn_enemies;
use systems::player_movement::player_movement;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(
            Startup,
            (setup, setup_resources, list_gamepads, play_theme).chain(),
        )
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
                update_damage_effect,
                systems::render::draw_scene,
                despawn_entities,
            )
                .chain(),),
        )
        .run();
}

fn play_theme(mut sound_manager: ResMut<SoundManager>) {
    sound_manager.play_theme(-17.0).unwrap();
}

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        1.25,
        TimerMode::Repeating,
    )));
    commands.insert_resource(ProjectileCooldownTimer(Timer::from_seconds(
        2.0,
        TimerMode::Once,
    )));
    commands.insert_resource(PlayerMovementTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
    commands.insert_resource(EnemyMovementTimer(Timer::from_seconds(
        0.35,
        TimerMode::Repeating,
    )));
    commands.insert_resource(DamageEffectTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(CameraOffset(IVec2::default()));
    commands.insert_resource(SoundManager::new(PathBuf::from("./assets/sfx/")).unwrap());
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([80, 50]), TerminalBorder::single_line()));
    commands.spawn((Player::new(IVec2::new(40, 25)), Transform::default()));
    commands.spawn(TerminalCamera::new());
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    println!("Looking for gamepads...");
    for name in &gamepads {
        println!("Found gamepad: {}", name.0);
    }
}
