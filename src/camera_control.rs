//! Control camera and allow for pan and zoom

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use iyes_loopless::prelude::*;

/// Marker for the main game camera
#[derive(Debug, Copy, Clone, Component)]
pub struct MainCamera;

/// Allow camera controls
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera);

        // ! ONLY ALLOW MOVING CAMERA IN GAMEPLAY MODE!
        app.add_system(
            move_camera
                .run_in_state(crate::MainState::Playing)
                .run_if_not(crate::state::is_paused),
        );
        app.add_system(
            zoom_camera
                .run_in_state(crate::MainState::Playing)
                .run_if_not(crate::state::is_paused),
        );
    }
}

/// Create a 2d camera to render scenes
fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::new_with_far(100.0))
        .insert(MainCamera);
}

/// Holds reference mouse position
#[derive(Default)]
struct ClickedMousePosition(Vec2);

/// Move camera when we hold down the scroll view!
fn move_camera(
    mut query: Query<&mut Transform, With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut reference_pos: Local<ClickedMousePosition>,
    windows: Res<Windows>,
) {
    let mut camera_location = query.single_mut();

    let window = windows.get_primary().unwrap();

    if let Some(mouse_location) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Right) {
            reference_pos.0 = mouse_location;
        } else if buttons.pressed(MouseButton::Right) {
            let offset = reference_pos.0 - mouse_location;
            let offset = offset * camera_location.scale.x;
            camera_location.translation += offset.extend(0.0);

            reference_pos.0 = mouse_location;
        }
    }
}

const MIN_ZOOM_LEVEL: f32 = 0.1;

/// Allow player to zoom
fn zoom_camera(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let mut camera_trans = query.single_mut();

    for event in scroll_events.iter() {
        let scroll_amount = -1.0 * event.y / 20.0;

        camera_trans.scale.x += scroll_amount;

        if camera_trans.scale.x < MIN_ZOOM_LEVEL {
            camera_trans.scale.x = MIN_ZOOM_LEVEL;
        }

        camera_trans.scale.y = camera_trans.scale.x;
    }
}
