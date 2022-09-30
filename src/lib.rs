//! Core game logic

#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_docs_in_private_items)]

mod game_grid;
mod state;

#[cfg(debug_assertions)]
mod debug_system;

use state::Main as MainState;
use state::Pause as PauseState;
use state::Turn as TurnState;

use bevy::{prelude::*, winit::WinitSettings};
use iyes_loopless::prelude::*;

/// Background color screen will be cleared with each frame.
const BACK_GROUND_COLOR: Color = Color::BLACK;

/// Main game plugin
#[derive(Debug, Clone, Copy)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.insert_resource(WinitSettings::game())
            .insert_resource(WindowDescriptor {
                title: String::from("TTD"),
                ..default()
            })
            .insert_resource(ClearColor(BACK_GROUND_COLOR));

        app.add_startup_system(create_camera);

        // Setup starting state
        app.add_loopless_state(state::Main::default());

        #[cfg(debug_assertions)]
        {
            app.add_plugin(debug_system::DebugPlugin);
        }
    }
}

/// Create a 2d camera to render scenes
fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::new_with_far(1000.0));
}
