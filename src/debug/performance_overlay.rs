use crate::events::*;
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

// todo: because of the way this is structured, you'll only be able to toggle the overlay
// inside of the menu or game over states, since those are the only states where the input system runs
// we may want to restructure this in the future so that the overlay can be toggled in-game as well
fn toggle_overlay(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    mut ui_events: EventReader<UiActionEvent>,
) {
    // todo: maybe this should be a configurable keybind?
    for event in ui_events.read() {
        match event {
            UiActionEvent::Info => {
                if let Ok(e) = q_root.single() {
                    // despawn the existing Perf UI
                    commands.entity(e).despawn();
                } else {
                    spawn_overlay(commands);
                }

                break;
            }
            _ => {}
        }
    }
}
