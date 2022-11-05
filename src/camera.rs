//! Control camera and allow for pan and zoom

use crate::{state::Main, ui::BOTTOM_PADDING};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

/// Marker for the main game camera
#[derive(Debug, Copy, Clone, Component)]
pub struct MainCamera;

/// Allow camera controls
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera);
        app.add_system(fit_map_to_camera.run_in_state(crate::MainState::Playing));
    }
}

/// Create a 2d camera to render scenes
fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

/// Scale camera so map is always at the edges
fn fit_map_to_camera(
    windows: Res<Windows>,
    assets: Res<crate::assets::MiscAssets>,
    asset_store: Res<bevy::asset::Assets<bevy_ecs_ldtk::LdtkAsset>>,
    current_level: Res<bevy_ecs_ldtk::LevelSelection>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    // without the `Without` these queries might (but should never) refere to the same transform, which would be bad
    mut world_query: Query<
        &mut Transform,
        (With<crate::ldtk_loader::WorldMarker>, Without<MainCamera>),
    >,
) {
    if current_level.is_changed() || windows.is_changed() {
        // Get window size
        let primary_window = windows.get_primary().unwrap();
        let window_height = primary_window.height() - BOTTOM_PADDING;
        let window_width = primary_window.width();

        // Get level dimensions
        let world_data = asset_store.get(&assets.ldtk_source_file).unwrap();
        let level_data = world_data.get_level(&current_level).unwrap();
        let level_height = level_data.px_hei as f32;
        let level_width = level_data.px_wid as f32;

        let mut trans = query.single_mut();

        let world_trans = world_query.get_single_mut();

        if let Ok(mut world_trans) = world_trans {
            // Scale Camera
            let height_scale = window_height / level_height;
            let width_scale = window_width / level_width;
            let scale = f32::min(height_scale, width_scale);
            world_trans.scale.x = scale;
            world_trans.scale.y = scale;

            // center camera
            trans.translation.x = level_width * scale / 2.;
            trans.translation.y = level_height * scale / 2.;
            trans.translation.y -= BOTTOM_PADDING / 2.;
        }
    }
}
