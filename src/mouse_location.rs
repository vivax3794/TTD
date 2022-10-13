//! Store World position of mouse

// ? Should this be a called each frame?
// ? Like it is convenient, but it might have some performance issues?
// ? One thing we can do is only run in "Playing" mode since that is the only times we should need world_position

use bevy::{prelude::*, render::camera::RenderTarget};
use iyes_loopless::prelude::*;

/// Store the mouses position in the world
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWorldPos(pub Vec2);

/// Keeps track of the mouses world position using a [`MouseWorldPos`] global resource
pub struct MouseWorldPlugin;

impl Plugin for MouseWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseWorldPos(Vec2::ZERO));
        app.add_system(update_world_position.run_in_state(crate::MainState::Playing));
    }
}

/// Update mouse world position
fn update_world_position(
    mut mouse_world_pos: ResMut<MouseWorldPos>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let gpu_space = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let gpu_to_world_matrix =
            camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = gpu_to_world_matrix.project_point3(gpu_space.extend(0.0));
        let world_pos = world_pos.truncate();

        mouse_world_pos.0 = world_pos;
    }
}
