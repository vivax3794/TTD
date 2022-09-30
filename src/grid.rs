//! Fix enemies and towers to a grid.

use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// TODO: Make this be based on the window size? should be simple enough!
/// We want grid cells to be squares, so we only define one side
const GRID_DIMENSIONS: f32 = 50.0;
/// How many cells in the Y should we have
const CELL_AMOUNT_Y: usize = 10;
/// How many cells in the x should we have?
const CELL_AMOUNT_X: usize = 20;

/// Data details entities on the grid
/// This must be kept in sync with the world, all despawns and spawns should edit this!
pub struct GridData(Vec<Vec<Option<Entity>>>);

impl Default for GridData {
    fn default() -> Self {
        Self(vec![vec![None; CELL_AMOUNT_Y]; CELL_AMOUNT_X])
    }
}

impl Index<(usize, usize)> for GridData {
    type Output = Option<Entity>;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[x][y]
    }
}

impl IndexMut<(usize, usize)> for GridData {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}

/// This plugin will make the [`gridPosition`] component do its job.
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(crate::MainState::Playing, create_new_grid);
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
                    let mut trans = query.get_mut(*entity).expect("DESYNC BETWEEN GRID AND ENTITIES");

                    trans.translation.x = (x - CELL_AMOUNT_X) as f32 * GRID_DIMENSIONS;
                    trans.translation.y = (y - CELL_AMOUNT_Y) as f32 * GRID_DIMENSIONS;
                }
            }
        }
    }
}