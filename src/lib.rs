//! Core game logic

#![warn(clippy::pedantic)]
// We often have stuff like `EnemyBundle` in the `enemy` module, and clippy doesnt like that
#![allow(clippy::module_name_repetitions)]
// Clippy doesn like passing Res<...> by value, but you cant take them as references in bevy
#![allow(clippy::needless_pass_by_value)]
// We need to do a lot of casting between i32 and f32 because we translate between world and grid positions
// And while clippy is right that we could lose precision, this only happens for really large values
// And we wont hit those large values in this project
// And we dont use a smaller data structure like i8 because the default for `bevy_esc_ldtk` is i32, so we will be using that
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
// We should implement these where we can for better debugging and performance
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// We will have many private functions in this project, and they should be documented so it is easier to work with
#![warn(clippy::missing_docs_in_private_items)]

use bevy::asset::AssetServerSettings;
use bevy::render::texture::ImageSettings;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_mod_ui_texture_atlas_image::UiAtlasImagePlugin;
use iyes_loopless::prelude::AppLooplessStateExt;

#[macro_use]
mod utils;
mod grid_position;

mod assets;
mod state;
mod turns;

mod camera;
mod mouse_location;

mod ldtk_loader;

mod enemies;
mod player;

#[cfg(feature = "debug_editor")]
mod debug_system;

use state::Main as MainState;
use state::RemoveOnGameplayExit;
use turns::{TurnPart, TurnState};

use camera::MainCamera;

/// Background color screen will be cleared with each frame.
const BACK_GROUND_COLOR: Color = Color::DARK_GRAY;

/// Main game plugin
#[derive(Debug, Clone, Copy)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Must be added before DefaultPlugins
        app.insert_resource(ImageSettings::default_nearest());
        // Asset Hot Reloading
        #[cfg(debug_assertions)]
        {
            app.insert_resource(AssetServerSettings {
                watch_for_changes: true,
                ..default()
            });
        }

        app.add_plugins(DefaultPlugins);

        // Window settings
        app.insert_resource(WinitSettings::game())
            .insert_resource(WindowDescriptor {
                title: String::from("TTD"),
                ..default()
            })
            .insert_resource(ClearColor(BACK_GROUND_COLOR));

        // Third party plugins
        app.add_plugin(bevy_prototype_lyon::prelude::ShapePlugin);
        app.add_plugin(UiAtlasImagePlugin);
        app.add_plugin(bevy_tweening::TweeningPlugin);

        // State
        app.add_loopless_state(MainState::LoadingAssets);
        app.add_loopless_state(TurnState::None);

        // Plugins
        app.add_plugin(state::StatePlugin);
        app.add_plugin(turns::TurnPlugin);
        // Tilemap must be before assets since it registers a asset loader
        app.add_plugin(ldtk_loader::LDtkMangerPlugin);

        // Asset must be after state as it registers state systems.
        app.add_plugin(assets::AssetLoadingPlugin);

        app.add_plugin(mouse_location::MouseWorldPlugin);
        app.add_plugin(camera::CameraPlugin);

        // Gameplay plugins
        app.add_plugin(enemies::EnemyPlugin);

        #[cfg(feature = "debug_editor")]
        {
            app.add_plugin(debug_system::DebugPlugin);
        }
    }
}
