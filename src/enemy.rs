//! Controls enemy

use bevy::{prelude::*, transform};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_test_enemy);
    }
}

/// All enmies will have this components
#[derive(Component, Default)]
pub struct EnemyMarker;

/// Bundle containing everything a enemy will need
#[derive(Bundle)]
pub struct EnemyBundle {
    /// Make us be able to reconise an enemy!
    _name: Name,

    /// Holds dynamic vector art
    #[bundle]
    _shape: ShapeBundle,

    /// Position on the grid, this will update Transform for us!
    grid_pos: crate::GridLocation,

}

impl Default for EnemyBundle {
    fn default() -> Self {
        let shape = GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 6,
                feature: shapes::RegularPolygonFeature::Radius(20.0),
                ..default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::color(Color::RED),
            },
            // X, Y will be overwriten
            Transform::from_xyz(0.0, 0.0, 10.0),
        );

        Self {
            _name: Name::new("Enemy"),
            _shape: shape,
            grid_pos: crate::GridLocation {x: 0, y: 0},
        }
    }
}

/// Just for testing
fn create_test_enemy(mut commands: Commands) {
    commands.spawn_bundle(EnemyBundle::default());
}

// MOVE ENEMIES
