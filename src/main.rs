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
use resources::game_state::GameState;
use resources::sound::SoundManager;
use resources::timers::{
    DamageEffectTimer, EnemyMovementTimer, EnemySpawnTimer, FadeTimer, LoadingTimer, PlayerMovementTimer,
    ProjectileCooldownTimer,
};
use std::path::*;
use systems::enemy_ai::enemy_ai;
use systems::enemy_spawn::spawn_enemies;
use systems::player_movement::player_movement;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ASCII Survivors".into(),
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            TerminalPlugins,
        ))
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (setup, setup_resources, list_gamepads).chain(),
        )
        .add_systems(OnEnter(GameState::Loading), show_window)
        .add_systems(OnEnter(GameState::FadingIn), (reset_fade_timer, play_start_sound).chain())
        .add_systems(OnEnter(GameState::Game), (setup_game, play_theme).chain())
        .add_systems(
            Update,
            (
                loading_render_system.run_if(in_state(GameState::Loading)),
                loading_update_system.run_if(in_state(GameState::Loading)),
                menu_input_system.run_if(in_state(GameState::Menu)),
                menu_render_system.run_if(in_state(GameState::Menu)),
                fade_in_render_system.run_if(in_state(GameState::FadingIn)),
                fade_in_update_system.run_if(in_state(GameState::FadingIn)),
                (
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
                    .chain()
                    .run_if(in_state(GameState::Game)),
            ),
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
    commands.insert_resource(LoadingTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    commands.insert_resource(FadeTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    commands.insert_resource(CameraOffset(IVec2::default()));
    commands.insert_resource(SoundManager::new(PathBuf::from("./assets/sfx/")).unwrap());
}

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([80, 50]), TerminalBorder::single_line()));
    commands.spawn(TerminalCamera::new());
}

fn setup_game(mut commands: Commands) {
    commands.spawn((Player::new(IVec2::new(40, 25)), Transform::default()));
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    println!("Looking for gamepads...");
    for name in &gamepads {
        println!("Found gamepad: {}", name.0);
    }
}

fn menu_render_system(mut query: Query<&mut Terminal>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();

        // Draw title
        let title = "ASCII SURVIVORS";
        let title_x = (80 - title.len()) / 2;
        terminal.put_string([title_x, 15], title);

        // Draw play button
        let button_text = "[ PLAY ]";
        let button_x = (80 - button_text.len()) / 2;
        terminal.put_string([button_x, 25], button_text);

        // Draw instructions
        let instruction = "Press SPACE or ENTER to start";
        let instruction_x = (80 - instruction.len()) / 2;
        terminal.put_string([instruction_x, 30], instruction);
    }
}

fn menu_input_system(
    input: Res<ButtonInput<KeyCode>>,
    _mouse_input: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::FadingIn);
    }
}

fn loading_render_system(mut query: Query<&mut Terminal>, loading_timer: Res<LoadingTimer>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();
        
        // Draw title
        let title = "ASCII SURVIVORS";
        let title_x = (80 - title.len()) / 2;
        terminal.put_string([title_x, 20], title);
        
        // Draw loading text
        let loading_text = "Loading...";
        let loading_x = (80 - loading_text.len()) / 2;
        terminal.put_string([loading_x, 25], loading_text);
        
        // Draw progress bar
        let progress = loading_timer.0.fraction();
        let bar_width = 40;
        let filled_width = (bar_width as f32 * progress) as usize;
        
        let bar_x = (80 - bar_width) / 2;
        
        // Draw progress bar background
        for i in 0..bar_width {
            if i < filled_width {
                terminal.put_char([bar_x + i, 27], '█');
            } else {
                terminal.put_char([bar_x + i, 27], '░');
            }
        }
        
        // Draw percentage
        let percentage = format!("{}%", (progress * 100.0) as u32);
        let percent_x = (80 - percentage.len()) / 2;
        terminal.put_string([percent_x, 29], percentage);
    }
}

fn loading_update_system(
    time: Res<Time>,
    mut loading_timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    loading_timer.0.tick(time.delta());
    
    if loading_timer.0.finished() {
        next_state.set(GameState::Menu);
    }
}

fn show_window(mut window_query: Query<&mut Window>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.visible = true;
    }
}

fn reset_fade_timer(mut fade_timer: ResMut<FadeTimer>) {
    fade_timer.0.reset();
}

fn play_start_sound(mut sound_manager: ResMut<SoundManager>) {
    let _ = sound_manager.play_sound(PathBuf::from("./assets/sfx/start.wav"), -5.0);
}

fn fade_in_render_system(mut query: Query<&mut Terminal>, fade_timer: Res<FadeTimer>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();
        
        // Calculate fade progress (0.0 = black screen, 1.0 = fully visible)
        let fade_progress = fade_timer.0.fraction();
        
        // Create a fade effect by gradually revealing more content
        let terminal_height = 50;
        let terminal_width = 80;
        
        // Start by filling the screen with gradually clearing characters
        let fade_char = if fade_progress < 0.3 {
            '█' // Solid block
        } else if fade_progress < 0.6 {
            '▓' // Dark shade
        } else if fade_progress < 0.9 {
            '▒' // Medium shade
        } else {
            '░' // Light shade
        };
        
        // Fill the screen with fade character, but reduce coverage as we progress
        let coverage = 1.0 - fade_progress;
        for y in 0..terminal_height {
            for x in 0..terminal_width {
                // Create a pattern that clears from center outward
                let center_x = terminal_width as f32 / 2.0;
                let center_y = terminal_height as f32 / 2.0;
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt();
                let normalized_distance = distance / max_distance;
                
                if normalized_distance < coverage {
                    terminal.put_char([x, y], fade_char);
                }
            }
        }
        
        // Show "Starting..." text during fade
        if fade_progress < 0.8 {
            let starting_text = "Starting...";
            let starting_x = (80 - starting_text.len()) / 2;
            terminal.put_string([starting_x, 25], starting_text);
        }
    }
}

fn fade_in_update_system(
    time: Res<Time>,
    mut fade_timer: ResMut<FadeTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    fade_timer.0.tick(time.delta());
    
    if fade_timer.0.finished() {
        next_state.set(GameState::Game);
    }
}
