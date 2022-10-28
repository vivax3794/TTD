//! Put boats  on enemies that are underwater

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::grid_position::GridPosition;
use crate::ldtk_loader::{TileType, get_tile_type_at};

