//! Used if debugging

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;

/// Debug Plugin
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin);
    }
}