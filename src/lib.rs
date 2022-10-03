//! Core game logic

// #![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_docs_in_private_items)]

mod assets;
mod state;
mod turns;

mod camera_control;
mod mouse_location;

mod enemies;
mod grid;

#[cfg(debug_assertions)]
mod debug_system;

use bevy::render::texture::ImageSettings;
use iyes_loopless::prelude::AppLooplessStateExt;
use state::Main as MainState;
use state::RemoveOnGameplayExit;
use turns::{TurnPart, TurnState};

use camera_control::MainCamera;
use grid::GridLocation;

use bevy::{prelude::*, winit::WinitSettings};

/// Background color screen will be cleared with each frame.
const BACK_GROUND_COLOR: Color = Color::BLACK;

/// Main game plugin
#[derive(Debug, Clone, Copy)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest());
        app.add_plugins(DefaultPlugins);
        app.insert_resource(WinitSettings::game())
            .insert_resource(WindowDescriptor {
                title: String::from("TTD"),
                ..default()
            })
            .insert_resource(ClearColor(BACK_GROUND_COLOR));

        app.add_plugin(bevy_prototype_lyon::prelude::ShapePlugin);

        app.add_loopless_state(MainState::LoadingAssets);
        app.add_loopless_state(TurnState::None);

        // Plugins
        app.add_plugin(state::StatePlugin);
        app.add_plugin(assets::AssetLoadingPlugin);
        app.add_plugin(turns::TurnPlugin);
        app.add_plugin(mouse_location::MouseWorldPlugin);
        app.add_plugin(camera_control::CameraPlugin);
        app.add_plugin(grid::GridPlugin);
        app.add_plugin(enemies::EnemyPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(debug_system::DebugPlugin);
        }
    }
}
