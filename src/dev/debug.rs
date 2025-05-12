use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[allow(dead_code)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()));
}