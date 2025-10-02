mod effects;
mod objects;
mod resources;
mod systems;

use crate::{effects::*, objects::{interaction::{Interaction, InteractionType}, *,}, resources::*, systems::*};
use crate::systems::cleanup::despawn_portals;
use bevy::{prelude::*, window::*};
use bevy_ascii_terminal::*;
use std::path::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ASCII Survivors".into(),
                    visible: false,
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }),
            TerminalPlugins,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_resources, list_gamepads).chain())
        .add_systems(OnEnter(GameState::Loading), show_window)
        .add_systems(
            OnEnter(GameState::FadingIn),
            (reset_fade_timer, play_start_sound).chain(),
        )
        .add_systems(OnEnter(GameState::Game), (setup_game, play_theme).chain())
        .add_systems(OnEnter(GameState::LevelTransition), (setup_level_transition, despawn_portals).chain())
        .add_systems(OnEnter(GameState::GameOver), stop_theme_music)
        .add_systems(
            Update,
            (
                (loading_render_system, loading_update_system).run_if(in_state(GameState::Loading)),
                (menu_input_system, menu_render_system).run_if(in_state(GameState::Menu)),
                (fade_in_render_system, fade_in_update_system)
                    .run_if(in_state(GameState::FadingIn)),
                (
                    player_movement,
                    spawn_enemies,
                    spawn_portal_after_survival,
                    spawn_shop_npcs_on_rest_level,
                    interaction_system,
                    heal_player_system,
                    portal_transition_system,
                    update_survival_timer,
                    (
                        enemy_ai,
                        auto_cast,
                        process_projectiles,
                        process_collisions,
                        orb_movement,
                        process_orb_collection,
                        campfire_animation_system,
                        ember_animation_system,
                    )
                        .chain(),
                    update_status_effect,
                    death_detection_system,
                    systems::render::render_system,
                    render_message_system,
                    render_portal_transition,
                    despawn_entities,
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
                (
                    heal_player_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
                (
                    level_transition_system,
                    level_transition_render_system,
                )
                    .run_if(in_state(GameState::LevelTransition)),
                (
                    game_over_input_system,
                    despawn_all_entities.run_if(in_state(GameState::GameOver)),
                    game_over_render_system,
                )
                    .run_if(in_state(GameState::GameOver)),
            ),
        )
        .run();
}

fn play_theme(mut sound_manager: ResMut<SoundManager>, level: Res<Level>) {
    if level.as_ref() == &Level::Survival {
        sound_manager.play_theme(-17.0).unwrap();
    }
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
    commands.insert_resource(SurvivalTimer(Timer::from_seconds(3600.0, TimerMode::Once)));
    commands.insert_resource(LevelTransitionTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    commands.insert_resource(InteractionTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(PortalTransition::default());
    commands.insert_resource(CameraOffset(IVec2::default()));
    commands.insert_resource(crate::resources::scene_lock::SceneLock::default());
    commands.insert_resource(crate::resources::ruleset::Ruleset::default());
    commands.insert_resource(crate::resources::level::Level::default()); // Add Level resource
    commands.insert_resource(crate::resources::kill_count::KillCount::default());
    commands.insert_resource(
        SoundManager::new(PathBuf::from("./assets/sfx/")).expect("failed to load manager"),
    );
}

fn setup(mut commands: Commands) {
    commands.spawn(Terminal::new([80, 50]));
    commands.spawn(TerminalCamera::new());
}

fn setup_game(
    mut commands: Commands,
    player_query: Query<&Player>
) {
    if player_query.is_empty() {
        commands.spawn((Player::new(IVec2::new(40, 25)), Transform::default()));
    }
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

        let title = "ASCII SURVIVORS";
        let title_x = (80 - title.len()) / 2;
        terminal.put_string([title_x, 15], title);

        let button_text = "[ PLAY ]";
        let button_x = (80 - button_text.len()) / 2;
        terminal.put_string([button_x, 25], button_text);

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

        let title = "ASCII SURVIVORS";
        let title_x = (80 - title.len()) / 2;
        terminal.put_string([title_x, 20], title);

        let loading_text = "Loading...";
        let loading_x = (80 - loading_text.len()) / 2;
        terminal.put_string([loading_x, 25], loading_text);

        let progress = loading_timer.0.fraction();
        let bar_width = 40;
        let filled_width = (bar_width as f32 * progress) as usize;

        let bar_x = (80 - bar_width) / 2;

        for i in 0..bar_width {
            if i < filled_width {
                terminal.put_char([bar_x + i, 27], '█');
            } else {
                terminal.put_char([bar_x + i, 27], '░');
            }
        }

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
    sound_manager
        .play_sound(PathBuf::from("./start.wav"), -5.0)
        .expect("Failed to play start sound");
}

fn fade_in_render_system(mut query: Query<&mut Terminal>, fade_timer: Res<FadeTimer>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();

        let fade_progress = fade_timer.0.fraction();

        let terminal_height = 50;
        let terminal_width = 80;

        let fade_char = if fade_progress < 0.3 {
            '█'
        } else if fade_progress < 0.6 {
            '▓'
        } else if fade_progress < 0.9 {
            '▒'
        } else {
            '░'
        };

        let coverage = 1.0 - fade_progress;
        for y in 0..terminal_height {
            for x in 0..terminal_width {
                let center_x = terminal_width as f32 / 2.0;
                let center_y = terminal_height as f32 / 2.0;
                let distance =
                    ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt();
                let normalized_distance = distance / max_distance;

                if normalized_distance < coverage {
                    terminal.put_char([x, y], fade_char);
                }
            }
        }

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

fn death_detection_system(
    player_query: Query<&Player>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player) = player_query.single()
        && player.health <= 0.0
    {
        next_state.set(GameState::GameOver);
    }
}

fn stop_theme_music(mut sound_manager: ResMut<SoundManager>) {
    sound_manager.stop_theme();
}

fn game_over_render_system(mut query: Query<&mut Terminal>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();

        let death_message = "YOU DIED!";
        let death_x = (80 - death_message.len()) / 2;
        terminal.put_string([death_x, 20], death_message);

        let restart_message = "Press R to Restart";
        let restart_x = (80 - restart_message.len()) / 2;
        terminal.put_string([restart_x, 25], restart_message);

        let menu_message = "Press ESC to return to Menu";
        let menu_x = (80 - menu_message.len()) / 2;
        terminal.put_string([menu_x, 27], menu_message);
    }
}

fn despawn_all_entities(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    orb_query: Query<Entity, With<Orb>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in orb_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn game_over_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut camera_offset: ResMut<CameraOffset>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        camera_offset.0 = IVec2::default();
        next_state.set(GameState::Game);
    } else if input.just_pressed(KeyCode::Escape) {
        camera_offset.0 = IVec2::default();
        next_state.set(GameState::Menu);
    }
}

fn update_survival_timer(time: Res<Time>, mut survival_timer: ResMut<SurvivalTimer>) {
    survival_timer.0.tick(time.delta());
}

fn setup_level_transition(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    orb_query: Query<Entity, With<Orb>>,
    mut player_query: Query<&mut Player>,
    mut camera_offset: ResMut<CameraOffset>,
    level: Res<Level>,
    mut scene_lock: ResMut<crate::resources::scene_lock::SceneLock>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in orb_query.iter() {
        commands.entity(entity).despawn();
    }
    
    if let Ok(mut player) = player_query.single_mut() {
        player.position = IVec2::new(40, 25);
        player.world_position = IVec2::new(40, 25);
    }
    
    camera_offset.0 = IVec2::default();
    
    if level.as_ref() == &Level::Rest {
        scene_lock.0 = true;
        let campfire_position = IVec2::new(40, 25);
        
        commands.spawn((
            Campfire::new(campfire_position),
            Interaction::new(InteractionType::Campfire),
            Transform::from_xyz(campfire_position.x as f32, campfire_position.y as f32, 0.0),
        ));
    } else {
        scene_lock.0 = false;
    }
}

fn level_transition_system(
    time: Res<Time>,
    mut transition_timer: ResMut<LevelTransitionTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    transition_timer.0.tick(time.delta());
    
    if transition_timer.0.finished() {
        next_state.set(GameState::Game);
        transition_timer.0.reset();
    }
}

fn level_transition_render_system(mut query: Query<&mut Terminal>) {
    if let Ok(mut terminal) = query.single_mut() {
        terminal.clear();
        
        let message = "Level Transition...";
        let message_x = (80 - message.len()) / 2;
        terminal.put_string([message_x, 25], message);
        
        let sub_message = "Entering new area...";
        let sub_x = (80 - sub_message.len()) / 2;
        terminal.put_string([sub_x, 27], sub_message);
    }
}
