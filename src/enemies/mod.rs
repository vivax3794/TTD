//! Controls enemy

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use iyes_loopless::prelude::AppLooplessStateExt;

mod slime;

pub use slime::EnemySlime;

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_enter_system(crate::MainState::Playing, create_test_enemy);
    }
}

/// All enmies will have this components
#[derive(Component, Default)]
pub struct EnemyMarker;

/// Bundle containing everything a enemy will need
#[derive(Bundle)]
pub struct EnemyBundle {
    /// Marker so we know this is a enemy
    _m: EnemyMarker,
    /// Make us be able to reconise an enemy!
    _name: Name,

    /// Holds dynamic vector art
    #[bundle]
    _sprite: SpriteSheetBundle,

    /// Position on the grid, this will update Transform for us!
    grid_pos: crate::GridLocation,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            _m: EnemyMarker,
            _name: Name::new("Enemy"),
            _sprite: SpriteSheetBundle::default(),
            grid_pos: crate::GridLocation { x: 0, y: 0 },
        }
    }
}

/// Just for testing
fn create_test_enemy(mut commands: Commands, assets: Res<crate::assets::EnemyAssets>) {
    commands.spawn_bundle(EnemySlime::with_assets(&assets.slime));
}

