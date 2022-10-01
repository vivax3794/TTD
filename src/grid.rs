//! Fix enemies and towers to a grid.

use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

// TODO: Make this be based on the window size? should be simple enough!
/// We want grid cells to be squares, so we only define one side
const GRID_DIMENSIONS: f32 = 50.0;
/// How many cells in the Y should we have
const CELL_AMOUNT_Y: usize = 5;
/// How many cells in the x should we have?
const CELL_AMOUNT_X: usize = 10;

/// Data details entities on the grid
/// This must be kept in sync with the world, all despawns and spawns should edit this!
pub struct GridData(pub Vec<Vec<Option<Entity>>>);

impl Default for GridData {
    fn default() -> Self {
        Self(vec![
            vec![None; CELL_AMOUNT_Y as usize];
            CELL_AMOUNT_X as usize
        ])
    }
}

/// Location on the mouse on the grid!
pub struct GridMouseLocation(usize, usize);

/// This plugin will make the [`gridPosition`] component do its job.
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridMouseLocation(0, 0));
        app.add_system(update_grid_mouse_location.run_in_state(crate::MainState::Playing));

        app.add_enter_system(crate::MainState::Playing, create_new_grid);
        app.add_enter_system(crate::MainState::Playing, draw_grid_lines);
        app.add_system(place_grid_entities.run_in_state(crate::MainState::Playing));
    }
}

/// Setup new grid
fn create_new_grid(mut commands: Commands) {
    commands.insert_resource(GridData::default());
}

/// Place grid entities in the current real world location!
fn place_grid_entities(grid: Res<GridData>, mut query: Query<&mut Transform>) {
    if grid.is_changed() {
        for (x, cols) in grid.0.iter().enumerate() {
            for (y, entity) in cols.iter().enumerate() {
                if let Some(entity) = entity {
                    let mut trans = query
                        .get_mut(*entity)
                        .expect("DESYNC BETWEEN GRID AND ENTITIES");

                    trans.translation.x = (x - CELL_AMOUNT_X) as f32 * GRID_DIMENSIONS;
                    trans.translation.y = (y - CELL_AMOUNT_Y) as f32 * GRID_DIMENSIONS;
                }
            }
        }
    }
}

/// Update grid mouse location based on the current mouse location
fn update_grid_mouse_location(
    mut grid_mouse_location: ResMut<GridMouseLocation>,
    world_mouse_location: Res<crate::mouse_location::MouseWorldPos>,
) {
    let world_pos = world_mouse_location.0;

    grid_mouse_location.0 = (world_pos.x * GRID_DIMENSIONS) as usize + CELL_AMOUNT_X;
    grid_mouse_location.1 = (world_pos.y * GRID_DIMENSIONS) as usize + CELL_AMOUNT_Y;
}

/// Spawn lines indicating where the grid is!
fn draw_grid_lines(mut commands: Commands) {
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(crate::RemoveOnGameplayExit)
        .with_children(|parent| {
            // Convenient function for spawning lines
            // Since we will be spawning the same type of line in both loops
            let mut create_line = |p1, p2| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(p1, p2),
                    DrawMode::Stroke(StrokeMode::new(Color::GRAY, 3.0)),
                    Transform::default(),
                ));
            };

            let col_length = GRID_DIMENSIONS * CELL_AMOUNT_Y as f32;
            for col in 0..CELL_AMOUNT_X * 2 + 1 {
                let col_x = (col as f32 - CELL_AMOUNT_X as f32) * GRID_DIMENSIONS;
                let p1 = Vec2::new(col_x, -col_length);
                let p2 = Vec2::new(col_x, col_length);
                create_line(p1, p2);
            }


            let row_length = GRID_DIMENSIONS * CELL_AMOUNT_X as f32;
            for col in 0..CELL_AMOUNT_Y * 2 + 1 {
                let row_y = (col as f32 - CELL_AMOUNT_Y as f32) * GRID_DIMENSIONS;
                let p1 = Vec2::new(-row_length, row_y);
                let p2 = Vec2::new(row_length, row_y);
                create_line(p1, p2);
            }
        });
}
