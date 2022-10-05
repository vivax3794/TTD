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
                translation: Vec3::new(256. / 2., 256. / 2., 100.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

/// Scale camera so map is always at the edges
fn fit_map_to_camera(windows: Res<Windows>, mut query: Query<&mut Transform, With<MainCamera>>) {
    let primary_window = windows.get_primary().unwrap();
    let height = primary_window.height();
    let width = primary_window.width();

    let mut trans = query.single_mut();
    
    // Real * Scale = World

    let scale_amount = 256. / f32::min(height, width);
    trans.scale.x = scale_amount;
    trans.scale.y = scale_amount;
    // dbg!(scale_amount);
}
