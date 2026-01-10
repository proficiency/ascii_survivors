use crate::{
    effects::update_status_effect,
    events::{AudioEvent, LevelChangedEvent, LevelUpEvent},
    maps,
    objects::{
        interaction::{Interaction, InteractionType},
        *,
    },
    plugins::audio::{AudioChannelType, AudioCommand},
    resources::*,
    spells::SpellType,
    systems::*,
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Gameplay,
    Rendering,
    Cleanup,
}

pub struct PlayerMovementPlugin;
pub struct PlayerCombatPlugin;
pub struct EnemyMovementPlugin;
pub struct EnemyCombatPlugin;
pub struct DespawnPlugin;
pub struct SpawnPlugin;
pub struct InteractionPlugin;
pub struct RenderingPlugin;
pub struct AmbientPlugin;
pub struct PlayerPlugin;
pub struct EnemyPlugin;
pub struct SchedulePlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(update_scene_lock)
                    .after(spawn_portal_after_survival)
                    .before(spawn_enemies),
                orb_movement
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(process_collisions),
                process_orb_collection
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(orb_movement),
            ),
        );
    }
}

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                auto_cast
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(boss_ai)
                    .after(portal_transition_system),
                process_projectiles
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(auto_cast),
            ),
        );
    }
}

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_ai
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(portal_transition_system)
                    .after(spawn_enemies)
                    .after(spawn_bosses),
                boss_ai
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(enemy_ai)
                    .after(spawn_bosses),
            ),
        );
    }
}

impl Plugin for EnemyCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_collisions
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(process_projectiles),
        );
    }
}

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                spawn_player,
                maps::map::load_map_system,
                |level: Res<Level>, mut level_changed_events: EventWriter<LevelChangedEvent>| {
                    // write level changed event
                    level_changed_events.write(LevelChangedEvent { new_level: *level });
                },
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                spawn_portal_after_survival,
                spawn_enemies,
                spawn_bosses,
                spawn_shop_npcs_on_rest_level,
            )
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay),
        );
    }
}

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_entities
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(render_portal_transition),
        );
    }
}

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interaction_system,
                apply_interaction_messages,
                heal_player_system,
                portal_transition_system,
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(spawn_shop_npcs_on_rest_level),
        );
    }
}

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_system,
                render_message_system,
                render_portal_transition,
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay)
                .after(update_status_effect),
        )
        .add_systems(
            Update,
            update_lighting_overlay
                .after(render_system)
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Rendering),
        );
    }
}

impl Plugin for AmbientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_scene_lock
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(spawn_portal_after_survival)
                    .before(player_movement),
                campfire_animation_system,
                ember_animation_system,
                light_flicker_system,
                update_status_effect.after(process_orb_collection),
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .in_set(GameSet::Gameplay),
        );
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerMovementPlugin, PlayerCombatPlugin))
            .add_systems(
                Update,
                level_up_system
                    .run_if(in_state(GameState::Game))
                    .in_set(GameSet::Gameplay)
                    .after(process_orb_collection),
            );
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyMovementPlugin, EnemyCombatPlugin));
    }
}

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionMessageEvent>()
            .add_event::<LevelChangedEvent>()
            .add_event::<LevelUpEvent>()
            .add_plugins((
                PlayerPlugin,
                EnemyPlugin,
                SpawnPlugin,
                DespawnPlugin,
                InteractionPlugin,
                RenderingPlugin,
                AmbientPlugin,
            ))
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Gameplay.after(GameSet::Input),
                    GameSet::Rendering.after(GameSet::Gameplay),
                    GameSet::Cleanup.after(GameSet::Rendering),
                ),
            )
            .add_systems(
                OnEnter(GameState::LevelTransition),
                (setup_level_transition, despawn_portals).chain(),
            )
            .add_systems(
                Update,
                (
                    update_survival_timer
                        .run_if(in_state(GameState::Game))
                        .in_set(GameSet::Gameplay)
                        .after(spawn_portal_after_survival),
                    death_detection_system
                        .run_if(in_state(GameState::Game))
                        .in_set(GameSet::Gameplay)
                        .after(update_status_effect),
                    level_transition_system
                        .run_if(in_state(GameState::LevelTransition))
                        .in_set(GameSet::Gameplay),
                ),
            );
    }
}

fn spawn_player(mut commands: Commands, player_query: Query<&Player>) {
    if player_query.is_empty() {
        let mut player = Player::new(IVec2::new(40, 25));
        player.arcanum.learn_spell(SpellType::Fireball);
        player.arcanum.learn_spell(SpellType::MagicMissile);
        commands.spawn((player, Transform::default()));
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
        // transition to game-over state
        next_state.set(GameState::GameOver);

        // stop the music
        audio_events.write(AudioEvent {
            channel: AudioChannelType::Music,
            command: AudioCommand::Stop,
        });
    }
}

fn update_survival_timer(time: Res<Time>, mut survival_timer: ResMut<SurvivalTimer>) {
    survival_timer.0.tick(time.delta());
}

#[allow(clippy::too_many_arguments)]
fn setup_level_transition(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    orb_query: Query<Entity, With<Orb>>,
    boss_query: Query<Entity, With<Boss>>,
    mut player_query: Query<&mut Player>,
    mut camera_offset: ResMut<CameraOffset>,
    mut cinematic_camera: ResMut<CinematicCamera>,
    level: Res<Level>,
) {
    // despawn
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
    for boss in boss_query.iter() {
        commands.entity(boss).despawn();
    }
    for projectile in projectile_query.iter() {
        commands.entity(projectile).despawn();
    }
    for orb in orb_query.iter() {
        commands.entity(orb).despawn();
    }

    // reposition the player and camera
    if let Ok(mut player) = player_query.single_mut() {
        player.position = IVec2::new(40, 25);
        player.world_position = IVec2::new(40, 25);
    }
    camera_offset.0 = IVec2::default();
    cinematic_camera.current_offset = Vec2::ZERO;
    cinematic_camera.target_offset = Vec2::ZERO;

    // spawn campfire on rest level
    if level.as_ref() == &Level::Rest {
        let campfire_position = IVec2::new(40, 25);

        commands.spawn((
            Campfire::new(campfire_position),
            Interaction::new(InteractionType::Campfire),
            LightEmitter::campfire(),
            LightFlicker::campfire(),
            Transform::from_xyz(campfire_position.x as f32, campfire_position.y as f32, 0.0),
        ));
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
