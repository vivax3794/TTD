//! Control camera and allow for pan and zoom

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

// TODO: We assume maps are 256x256 px atm, look into dynamic sizes later

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
) {
    if current_level.is_changed() || windows.is_changed() {
        let primary_window = windows.get_primary().unwrap();
        let window_height = primary_window.height();
        let window_width = primary_window.width();

        let world_data = asset_store.get(&assets.ldtk_source_file).unwrap();
        let level_data = world_data.get_level(&current_level).unwrap();
        let level_height = level_data.px_hei as f32;
        let level_width = level_data.px_wid as f32;

        let mut trans = query.single_mut();

        // center camera
        trans.translation.x = level_width / 2.;
        trans.translation.y = level_height / 2.;

        // Scale Camera 
        let height_scale = level_height / window_height;
        let width_scale = level_width / window_width;
        let scale = f32::max(height_scale, width_scale);
        trans.scale.x = scale;
        trans.scale.y = scale;
    }
}
