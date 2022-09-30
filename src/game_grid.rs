//! Fix enemies and towers to a grid.

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// TODO: Make this be based on the window size? should be simple enough!
/// We want grid cells to be squares, so we only define one side
const GRID_DIMENSIONS: f32 = 50.0;
/// How many cells in the Y should we have
const CELL_AMOUNT_Y: f32 = 10.0;
/// How many cells in the x should we have?
const CELL_AMOUNT_X: f32 = 20.0;

/// Will position entity in the world based on grid location.
#[derive(Debug, Clone, Copy, Component)]
pub struct GridPosition(pub Vec2);

/// This plugin will make the [`gridPosition`] component do its job.
struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(center_entities_on_grid.run_in_state(crate::MainState::Playing));
    }
}

/// Center all entities to the grid
fn center_entities_on_grid(
    mut query: Query<(&mut Transform, &GridPosition), Changed<GridPosition>>,
) {
    query.for_each_mut(|(mut trans, GridPosition(grid_pos))| {
        if !(-CELL_AMOUNT_X..CELL_AMOUNT_X).contains(&grid_pos.x)
            && !(-CELL_AMOUNT_Y..CELL_AMOUNT_Y).contains(&grid_pos.y)
        {
            eprintln!("entity not on grid: {:?}", grid_pos);
            return;
        }

        let offset = Vec2::new(CELL_AMOUNT_X, CELL_AMOUNT_Y);
        let world_pos = (*grid_pos - offset) * GRID_DIMENSIONS;

        // We need to preserve Z
        trans.translation.x = world_pos.x;
        trans.translation.y = world_pos.y;
    });
}
