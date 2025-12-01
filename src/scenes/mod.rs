use std::collections::HashMap;

use bevy::{
    prelude::*,
    scene::{DynamicScene, InstanceId, SceneSpawner},
};

use bevy_ascii_terminal::Terminal;

use crate::resources::{FadeTimer, GameState, LoadingTimer};

const TERMINAL_WIDTH: usize = 80;
const TERMINAL_HEIGHT: usize = 50;

pub struct GameScenesPlugin;

impl Plugin for GameScenesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SceneMarker>()
            .register_type::<SceneText>()
            .register_type::<SceneProgressBar>()
            .register_type::<SceneFadeOverlay>()
            .init_resource::<SceneAssets>()
            .init_resource::<ActiveSceneInstances>()
            .add_systems(Startup, setup_scene_assets)
            .add_systems(Update, render_scene_overlays)
            .add_systems(OnEnter(GameState::Loading), spawn_loading_scene)
            .add_systems(OnExit(GameState::Loading), despawn_loading_scene)
            .add_systems(OnEnter(GameState::Menu), spawn_menu_scene)
            .add_systems(OnExit(GameState::Menu), despawn_menu_scene)
            .add_systems(OnEnter(GameState::FadingIn), spawn_fade_scene)
            .add_systems(OnExit(GameState::FadingIn), despawn_fade_scene)
            .add_systems(
                OnEnter(GameState::LevelTransition),
                spawn_level_transition_scene,
            )
            .add_systems(
                OnExit(GameState::LevelTransition),
                despawn_level_transition_scene,
            )
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_scene)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_scene);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum SceneId {
    Loading,
    Menu,
    FadeIn,
    LevelTransition,
    GameOver,
}

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct SceneMarker {
    pub id: SceneId,
}

impl SceneMarker {
    pub fn new(id: SceneId) -> Self {
        Self { id }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct SceneText {
    pub value: String,
    pub row: i32,
    pub column: i32,
    pub centered: bool,
}

impl SceneText {
    pub fn centered(row: i32, value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            row,
            column: 0,
            centered: true,
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct SceneProgressBar {
    pub row: i32,
    pub width: usize,
    pub label_row: i32,
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct SceneFadeOverlay {
    pub width: usize,
    pub height: usize,
}

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub loading: Option<Handle<DynamicScene>>,
    pub menu: Option<Handle<DynamicScene>>,
    pub fade_in: Option<Handle<DynamicScene>>,
    pub level_transition: Option<Handle<DynamicScene>>,
    pub game_over: Option<Handle<DynamicScene>>,
}

impl SceneAssets {
    fn get(&self, id: SceneId) -> Option<Handle<DynamicScene>> {
        match id {
            SceneId::Loading => self.loading.clone(),
            SceneId::Menu => self.menu.clone(),
            SceneId::FadeIn => self.fade_in.clone(),
            SceneId::LevelTransition => self.level_transition.clone(),
            SceneId::GameOver => self.game_over.clone(),
        }
    }
}

#[derive(Resource, Default)]
pub struct ActiveSceneInstances {
    instances: HashMap<SceneId, InstanceId>,
}

impl ActiveSceneInstances {
    fn activate(&mut self, id: SceneId, instance: InstanceId) {
        self.instances.insert(id, instance);
    }

    fn remove(&mut self, id: SceneId) -> Option<InstanceId> {
        self.instances.remove(&id)
    }
}

fn setup_scene_assets(
    mut scenes: ResMut<Assets<DynamicScene>>,
    mut assets: ResMut<SceneAssets>,
    type_registry: Res<AppTypeRegistry>,
) {
    let registry = type_registry.as_ref();
    assets.loading = Some(scenes.add(build_loading_scene(registry)));
    assets.menu = Some(scenes.add(build_menu_scene(registry)));
    assets.fade_in = Some(scenes.add(build_fade_scene(registry)));
    assets.level_transition = Some(scenes.add(build_level_transition_scene(registry)));
    assets.game_over = Some(scenes.add(build_game_over_scene(registry)));
}

fn spawn_loading_scene(
    handles: Res<SceneAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    spawn_scene_instance(
        SceneId::Loading,
        &handles,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn despawn_loading_scene(
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    despawn_scene_instance(SceneId::Loading, &mut scene_spawner, &mut active_scenes);
}

fn spawn_menu_scene(
    handles: Res<SceneAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    spawn_scene_instance(
        SceneId::Menu,
        &handles,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn despawn_menu_scene(
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    despawn_scene_instance(SceneId::Menu, &mut scene_spawner, &mut active_scenes);
}

fn spawn_fade_scene(
    handles: Res<SceneAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    spawn_scene_instance(
        SceneId::FadeIn,
        &handles,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn despawn_fade_scene(
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    despawn_scene_instance(SceneId::FadeIn, &mut scene_spawner, &mut active_scenes);
}

fn spawn_level_transition_scene(
    handles: Res<SceneAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    spawn_scene_instance(
        SceneId::LevelTransition,
        &handles,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn despawn_level_transition_scene(
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    despawn_scene_instance(
        SceneId::LevelTransition,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn spawn_game_over_scene(
    handles: Res<SceneAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    spawn_scene_instance(
        SceneId::GameOver,
        &handles,
        &mut scene_spawner,
        &mut active_scenes,
    );
}

fn despawn_game_over_scene(
    mut scene_spawner: ResMut<SceneSpawner>,
    mut active_scenes: ResMut<ActiveSceneInstances>,
) {
    despawn_scene_instance(SceneId::GameOver, &mut scene_spawner, &mut active_scenes);
}

fn spawn_scene_instance(
    id: SceneId,
    handles: &SceneAssets,
    spawner: &mut SceneSpawner,
    active_scenes: &mut ActiveSceneInstances,
) {
    if let Some(handle) = handles.get(id) {
        let instance_id = spawner.spawn_dynamic(handle);
        active_scenes.activate(id, instance_id);
    } else {
        warn!("attempted to spawn scene {:?} before assets were ready", id);
    }
}

fn despawn_scene_instance(
    id: SceneId,
    spawner: &mut SceneSpawner,
    active_scenes: &mut ActiveSceneInstances,
) {
    if let Some(instance) = active_scenes.remove(id) {
        spawner.despawn_instance(instance);
    }
}

fn render_scene_overlays(
    mut terminal_query: Query<&mut Terminal>,
    text_query: Query<&SceneText>,
    progress_query: Query<&SceneProgressBar>,
    fade_query: Query<&SceneFadeOverlay>,
    loading_timer: Option<Res<LoadingTimer>>,
    fade_timer: Option<Res<FadeTimer>>,
) {
    let has_scene = !text_query.is_empty() || !progress_query.is_empty() || !fade_query.is_empty();
    if !has_scene {
        return;
    }

    let Ok(mut terminal) = terminal_query.single_mut() else {
        return;
    };

    terminal.clear();

    if let Some(overlay) = fade_query.iter().next() {
        if let Some(progress) = fade_timer.as_ref().map(|timer| timer.0.fraction()) {
            render_fade_overlay(&mut terminal, overlay, progress);
        }
    }

    for text in &text_query {
        render_scene_text(&mut terminal, text);
    }

    if let Some(progress_bar) = progress_query.iter().next() {
        if let Some(progress) = loading_timer.as_ref().map(|timer| timer.0.fraction()) {
            render_progress_bar(&mut terminal, progress_bar, progress);
        }
    }
}

fn render_scene_text(terminal: &mut Terminal, text: &SceneText) {
    let row = text.row.max(0) as usize;
    let column = if text.centered {
        ((TERMINAL_WIDTH as i32 - text.value.len() as i32) / 2).max(0) as usize
    } else {
        text.column.max(0) as usize
    };
    terminal.put_string([column, row], text.value.as_str());
}

fn render_progress_bar(terminal: &mut Terminal, bar: &SceneProgressBar, progress: f32) {
    let row = bar.row.max(0) as usize;
    let start_x = ((TERMINAL_WIDTH as i32 - bar.width as i32) / 2).max(0) as usize;
    let filled = (progress.clamp(0.0, 1.0) * bar.width as f32) as usize;

    for x in 0..bar.width {
        let ch = if x < filled { '#' } else { '-' };
        terminal.put_char([start_x + x, row], ch);
    }

    let percent = format!("{:.0}%", (progress * 100.0).clamp(0.0, 100.0));
    let percent_column = ((TERMINAL_WIDTH as i32 - percent.len() as i32) / 2).max(0) as usize;
    let label_row = bar.label_row.max(0) as usize;
    terminal.put_string([percent_column, label_row], percent.as_str());
}

fn render_fade_overlay(terminal: &mut Terminal, overlay: &SceneFadeOverlay, progress: f32) {
    let fade_char = if progress < 0.3 {
        '#'
    } else if progress < 0.6 {
        '*'
    } else if progress < 0.9 {
        '.'
    } else {
        ' '
    };

    let coverage = 1.0 - progress;
    let width = overlay.width.max(1) as f32;
    let height = overlay.height.max(1) as f32;

    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt();

    for y in 0..overlay.height {
        for x in 0..overlay.width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let normalized = distance / max_distance;
            if normalized < coverage {
                terminal.put_char([x, y], fade_char);
            }
        }
    }

    if progress < 0.8 {
        let text = "Starting...";
        let column = ((TERMINAL_WIDTH as i32 - text.len() as i32) / 2).max(0) as usize;
        let row = (TERMINAL_HEIGHT / 2) as usize;
        terminal.put_string([column, row], text);
    }
}

fn build_loading_scene(type_registry: &AppTypeRegistry) -> DynamicScene {
    build_dynamic_scene(type_registry, |world| {
        world.spawn((
            SceneMarker::new(SceneId::Loading),
            SceneText::centered(20, "ASCII SURVIVORS"),
        ));
        world.spawn((
            SceneMarker::new(SceneId::Loading),
            SceneText::centered(25, "Loading..."),
        ));
        world.spawn((
            SceneMarker::new(SceneId::Loading),
            SceneProgressBar {
                row: 27,
                width: 40,
                label_row: 29,
            },
        ));
    })
}

fn build_menu_scene(type_registry: &AppTypeRegistry) -> DynamicScene {
    build_dynamic_scene(type_registry, |world| {
        world.spawn((
            SceneMarker::new(SceneId::Menu),
            SceneText::centered(15, "ASCII SURVIVORS"),
        ));
        world.spawn((
            SceneMarker::new(SceneId::Menu),
            SceneText::centered(25, "[ PLAY ]"),
        ));
        world.spawn((
            SceneMarker::new(SceneId::Menu),
            SceneText::centered(30, "Press SPACE or ENTER to start"),
        ));
    })
}

fn build_fade_scene(type_registry: &AppTypeRegistry) -> DynamicScene {
    build_dynamic_scene(type_registry, |world| {
        world.spawn((
            SceneMarker::new(SceneId::FadeIn),
            SceneFadeOverlay {
                width: TERMINAL_WIDTH,
                height: TERMINAL_HEIGHT,
            },
        ));
    })
}

fn build_level_transition_scene(type_registry: &AppTypeRegistry) -> DynamicScene {
    build_dynamic_scene(type_registry, |world| {
        world.spawn((
            SceneMarker::new(SceneId::LevelTransition),
            SceneText::centered(25, "Level Transition..."),
        ));
        world.spawn((
            SceneMarker::new(SceneId::LevelTransition),
            SceneText::centered(27, "Entering new area..."),
        ));
    })
}

fn build_game_over_scene(type_registry: &AppTypeRegistry) -> DynamicScene {
    build_dynamic_scene(type_registry, |world| {
        world.spawn((
            SceneMarker::new(SceneId::GameOver),
            SceneText::centered(20, "YOU DIED!"),
        ));
        world.spawn((
            SceneMarker::new(SceneId::GameOver),
            SceneText::centered(25, "Press R to Restart"),
        ));
        world.spawn((
            SceneMarker::new(SceneId::GameOver),
            SceneText::centered(27, "Press ESC to return to Menu"),
        ));
    })
}

fn build_dynamic_scene(
    type_registry: &AppTypeRegistry,
    build_fn: impl FnOnce(&mut World),
) -> DynamicScene {
    let mut world = World::new();
    world.insert_resource(type_registry.clone());
    build_fn(&mut world);
    DynamicScene::from_world(&world)
}
