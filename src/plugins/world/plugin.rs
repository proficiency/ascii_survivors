use bevy::prelude::*;

use crate::{
    maps,
    objects::{Boss, Enemy, Orb, PlayerBundle, PlayerTag, Projectile},
    plugins::core::GameSet,
    plugins::enemy::{spawn_bosses, spawn_enemies},
    plugins::world::{
        cleanup::despawn_entities,
        portal_spawn::spawn_portal_after_survival,
        portal_transition::render_portal_transition, shop_npc_spawn::spawn_shop_npcs_on_rest_level,
    },
    resources::{AsciiGrid, CameraOffset, CinematicCamera, GameState, Level},
    spells::SpellType,
};

pub struct SpawnPlugin;
pub struct DespawnPlugin;
pub struct WorldPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                spawn_player,
                maps::map::load_map_system,
                write_level_changed_event,
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

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnPlugin, DespawnPlugin));
    }
}

fn spawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerTag>>,
    grid: Res<AsciiGrid>,
    mut cinematic: ResMut<CinematicCamera>,
    mut camera_offset: ResMut<CameraOffset>,
) {
    if player_query.is_empty() {
        let screen_center = IVec2::new(
            (grid.grid_size.x / 2) as i32,
            (grid.grid_size.y / 2) as i32,
        );
        let world_pos = IVec2::new(40, 25);
        let initial_offset = (world_pos - screen_center).as_vec2();
        cinematic.current_offset = initial_offset;
        cinematic.target_offset = initial_offset;
        camera_offset.0 = IVec2::new(
            initial_offset.x.round() as i32,
            initial_offset.y.round() as i32,
        );

        let mut bundle = PlayerBundle::at_screen(screen_center, world_pos);
        bundle.arcanum.learn_spell(SpellType::Fireball);
        bundle.arcanum.learn_spell(SpellType::MagicMissile);
        commands.spawn(bundle);
    }
}

fn write_level_changed_event(
    level: Res<Level>,
    mut events: EventWriter<crate::events::LevelChangedEvent>,
) {
    events.write(crate::events::LevelChangedEvent { new_level: *level });
}

#[allow(clippy::too_many_arguments)]
pub fn setup_level_transition(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    orb_query: Query<Entity, With<Orb>>,
    boss_query: Query<Entity, With<Boss>>,
    mut player_query: Query<&mut crate::objects::GridPosition, With<PlayerTag>>,
    mut camera_offset: ResMut<CameraOffset>,
    mut cinematic_camera: ResMut<CinematicCamera>,
    level: Res<Level>,
    grid: Res<AsciiGrid>,
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
    let screen_center = IVec2::new(
        (grid.grid_size.x / 2) as i32,
        (grid.grid_size.y / 2) as i32,
    );
    if let Ok(mut player) = player_query.single_mut() {
        player.tile = screen_center;
        player.world = IVec2::new(40, 25);
    }
    camera_offset.0 = IVec2::default();
    cinematic_camera.current_offset = Vec2::ZERO;
    cinematic_camera.target_offset = Vec2::ZERO;

    // spawn campfire on rest level
    if level.as_ref() == &Level::Rest {
        let campfire_position = IVec2::new(40, 25);

        commands.spawn((
            crate::objects::Campfire::new(campfire_position),
            crate::objects::Interaction::new(crate::objects::InteractionType::Campfire),
            crate::objects::LightEmitter::campfire(),
            crate::objects::LightFlicker::campfire(),
            Transform::from_xyz(campfire_position.x as f32, campfire_position.y as f32, 0.0),
        ));
    }
}

pub fn level_transition_system(
    time: Res<Time>,
    mut transition_timer: ResMut<crate::resources::LevelTransitionTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    transition_timer.0.tick(time.delta());
    if transition_timer.0.finished() {
        next_state.set(GameState::Game);
        transition_timer.0.reset();
    }
}
