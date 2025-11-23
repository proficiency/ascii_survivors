mod effects;
mod maps;
mod objects;
mod resources;
mod scenes;
mod spells;
mod systems;

use crate::{
    effects::*,
    objects::{interaction::InteractionType, *},
    resources::*,
    scenes::GameScenesPlugin,
    spells::*,
    systems::*,
};

use bevy::{prelude::*, window::*};
use bevy_ascii_terminal::*;
use bevy_kira_audio::prelude::*;

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
            AudioPlugin,
            GameScenesPlugin,
        ))
        .init_state::<GameState>()
        .add_audio_channel::<Music>()
        .add_audio_channel::<Sfx>()
        .insert_resource(crate::systems::spell_casting::SpellInputTimer::default())
        .add_systems(
            Startup,
            (
                setup,
                setup_resources,
                list_gamepads,
                setup_lighting_overlay,
            )
                .chain(),
        )
        .add_systems(OnEnter(GameState::Loading), show_window)
        .add_systems(
            OnEnter(GameState::FadingIn),
            (reset_fade_timer, play_start_sound).chain(),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (setup_game, play_theme, maps::map::load_map_system).chain(),
        )
        .add_systems(
            OnEnter(GameState::LevelTransition),
            (setup_level_transition, despawn_portals).chain(),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            |music_channel: Res<AudioChannel<Music>>| {
                music_channel.stop();
            },
        )
        .add_systems(
            Update,
            (
                loading_update_system.run_if(in_state(GameState::Loading)),
                menu_input_system.run_if(in_state(GameState::Menu)),
                fade_in_update_system.run_if(in_state(GameState::FadingIn)),
                (
                    player_movement,
                    spawn_enemies,
                    spawn_bosses,
                    spawn_portal_after_survival,
                    spawn_shop_npcs_on_rest_level,
                    interaction_system,
                    heal_player_system,
                    portal_transition_system,
                    update_survival_timer,
                    (
                        enemy_ai,
                        boss_ai,
                        auto_cast,
                        process_projectiles,
                        process_fireballs,
                        process_collisions,
                        orb_movement,
                        process_orb_collection,
                        campfire_animation_system,
                        ember_animation_system,
                        light_flicker_system,
                    )
                        .chain(),
                    update_status_effect,
                    death_detection_system,
                    spell_casting_system,
                    systems::render::render_system,
                    spell_render_system,
                    render_message_system,
                    render_portal_transition,
                    despawn_entities,
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
                level_transition_system.run_if(in_state(GameState::LevelTransition)),
                (game_over_input_system, despawn_all_entities)
                    .run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_systems(
            Update,
            update_lighting_overlay
                .after(render_system)
                .run_if(in_state(GameState::Game)),
        )
        .run();
}

fn play_theme(asset_server: Res<AssetServer>, audio: Res<AudioChannel<Music>>, level: Res<Level>) {
    if level.as_ref() == &Level::Survival {
        audio
            .play(asset_server.load("sfx/harmony.ogg"))
            .with_volume(0.1)
            .looped();
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
    commands.insert_resource(LevelTransitionTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
    commands.insert_resource(InteractionTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(PortalTransition::default());
    commands.insert_resource(CameraOffset(IVec2::default()));
    commands.insert_resource(crate::resources::scene_lock::SceneLock::default());
    commands.insert_resource(crate::resources::ruleset::Ruleset::default());
    commands.insert_resource(crate::resources::kill_count::KillCount::default());
    commands.insert_resource(Level::Grassland);
}

fn setup(mut commands: Commands) {
    commands.spawn(Terminal::new([80, 50]));
    commands.spawn(TerminalCamera::new());
}

fn setup_game(mut commands: Commands, player_query: Query<&Player>) {
    if player_query.is_empty() {
        let mut player = Player::new(IVec2::new(40, 25));
        player.arcanum.learn_spell(SpellType::Fireball);
        commands.spawn((player, Transform::default()));
    }
}

fn list_gamepads(gamepads: Query<(&Name, &Gamepad)>) {
    println!("Looking for gamepads...");
    for name in &gamepads {
        println!("Found gamepad: {}", name.0);
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

fn play_start_sound(asset_server: Res<AssetServer>, audio: Res<AudioChannel<Sfx>>) {
    audio
        .play(asset_server.load("sfx/start.wav"))
        .with_volume(0.5);
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
            crate::objects::Interaction::new(InteractionType::Campfire),
            LightEmitter::campfire(),
            LightFlicker::campfire(),
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
