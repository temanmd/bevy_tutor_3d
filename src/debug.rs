use bevy::prelude::*;
use iyes_perf_ui::{PerfUiPlugin, prelude::PerfUiDefaultEntries};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_debug_ui);
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
        app.add_plugins(PerfUiPlugin);
    }
}

fn add_debug_ui(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}
