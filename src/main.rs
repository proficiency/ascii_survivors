mod debug;
mod effects;
mod maps;
mod objects;
mod plugins;
mod resources;
mod scenes;
mod spells;
mod systems;

use crate::{
    debug::DebugPlugins,
    effects::*,
    objects::*,
    resources::*,
    scenes::*,
    spells::*,
    systems::*,
    {audio::*, bootstrap::*, plugins::*},
};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            BootstrapPlugin, // spawn the window, terminal, and resources
            AudioManagerPlugin,
            GameScenesPlugin,
            #[cfg(debug_assertions)]
            DebugPlugins,
        ))
        .add_systems(
            OnEnter(GameState::FadingIn),
            (reset_fade_timer, play_start_sound).chain(),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (spawn_player, maps::map::load_map_system).chain(),
        )
        .add_systems(
            OnEnter(GameState::LevelTransition),
            (setup_level_transition, despawn_portals).chain(),
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

fn spawn_player(mut commands: Commands, player_query: Query<&Player>) {
    if player_query.is_empty() {
        let mut player = Player::new(IVec2::new(40, 25));
        player.arcanum.learn_spell(SpellType::Fireball);
        commands.spawn((player, Transform::default()));
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

fn reset_fade_timer(mut fade_timer: ResMut<FadeTimer>) {
    fade_timer.0.reset();
}

fn play_start_sound(mut audio_events: EventWriter<AudioEvent>) {
    audio_events.write(AudioEvent {
        channel: AudioChannelType::Sfx,
        command: AudioCommand::Play {
            audio: "sfx/start.wav",
            looped: false,
            volume: Some(0.5),
        },
    });
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
    mut audio_events: EventWriter<AudioEvent>,
) {
    if let Ok(player) = player_query.single()
        && player.health <= 0.0
    {
        next_state.set(GameState::GameOver);
        audio_events.write(AudioEvent {
            channel: AudioChannelType::Music,
            command: AudioCommand::Stop,
        });
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
    mut scene_lock: ResMut<SceneLock>,
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
            crate::objects::Interaction::new(InteractionType::Campfire), // todo: maybe we should reconsider naming it 'Interaction'
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
