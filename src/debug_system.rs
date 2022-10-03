//! Used if debugging

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;
use iyes_loopless::state::CurrentState;

/// Debug Plugin
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin);

        app.add_system(log_state_changes);
    }
}


fn log_state_changes(main_state: Res<CurrentState<crate::MainState>>, turn_state: Res<CurrentState<crate::TurnState>>) {
    if main_state.is_changed() {
        dbg!(main_state.0);
    }
    if turn_state.is_changed() {
        dbg!(turn_state.0);
    }
}