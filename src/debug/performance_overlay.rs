use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, spawn_overlay)
            .add_systems(
                Update,
                toggle_overlay.before(iyes_perf_ui::PerfUiSet::Setup),
            );
    }
}

fn spawn_overlay(mut commands: Commands) {
    commands.spawn((
        // when we have lots of entries, we have to group them
        // into tuples, because of Bevy Rust syntax limitations.
        // we can eventually add more ascii_survivors-specific metrics as well.
        (
            PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
            PerfUiWidgetBar::new(PerfUiEntryFPSAverage::default()),
            PerfUiWidgetBar::new(PerfUiEntryFPSWorst::default()),
            PerfUiWidgetBar::new(PerfUiEntryFrameTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryCpuUsage::default()),
            PerfUiWidgetBar::new(PerfUiEntryMemUsage::default()),
            PerfUiWidgetBar::new(PerfUiEntryRenderCpuTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryRenderGpuTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryEntityCount::default()),
            PerfUiEntryFixedTimeStep::default(),
            PerfUiEntryRunningTime::default(),
            PerfUiEntryFrameCount::default(),
            PerfUiEntryCursorPosition::default(),
        ),
        (PerfUiEntryWindowResolution::default(),),
    ));
}

fn toggle_overlay(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    // todo: maybe this should be a configurable keybind?
    if kbd.just_pressed(KeyCode::Insert) {
        if let Ok(e) = q_root.single() {
            // despawn the existing Perf UI
            commands.entity(e).despawn();
        } else {
            spawn_overlay(commands);
        }
    }
}
