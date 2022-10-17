//! Version of `GridCoords` that uses a `IVec2` instead

use bevy::prelude::{Component, IVec2};
use bevy_ecs_ldtk::GridCoords;
use derive_more::{Add, AddAssign, From, Sub, SubAssign};

/// Custom gird position tracker
#[derive(Debug, Copy, Clone, Default, Component, Add, Sub, From, AddAssign, SubAssign)]
pub struct GridPosition(pub IVec2);

impl From<GridCoords> for GridPosition {
    fn from(source: GridCoords) -> Self {
        Self(source.into())
    }
}

impl From<GridPosition> for IVec2 {
    fn from(source: GridPosition) -> Self {
        source.0
    }
}
