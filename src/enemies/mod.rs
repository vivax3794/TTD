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

/// Marker for a eye entity
#[derive(Component)]
struct EyeMarker;

/// Describes where the eyes are, and how big they are.
#[derive(Component, Default, Debug)]
struct EyeSettings {
    /// X of the center of the eye
    offset_x: f32,
    /// Y of the center corner of the eye
    offset_y: f32,

    /// How wide is the eye?
    width: f32,
    /// How high is the eye?
    height: f32,
}

/// Eyes follow the mouse cursor!
#[derive(Bundle)]
struct EyesBundle {
    /// Marker to identitfy this entity as an eye
    _m: EyeMarker,

    /// Contains a basic black square
    #[bundle]
    shape: ShapeBundle,

    /// Settings for eye
    settings: EyeSettings,
}

impl Default for EyesBundle {
    fn default() -> Self {
        Self {
            _m: EyeMarker,
            settings: EyeSettings::default(),
            shape: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::splat(crate::assets::ASSET_SCALE_UP),
                    origin: RectangleOrigin::Center
                },
                DrawMode::Fill(FillMode::color(Color::BLACK)),
                Transform::default(),
            )
        }
    }
}


/// Describes a trait implenting the needed function for creating a enemy
trait EnemyConstruction : Bundle{
    /// Create the bundle with assets
    fn with_assets(assets: &Handle<TextureAtlas>) -> Self;
    /// Get eye settings for the eye enteties 
    fn eye_settings() -> Vec<EyeSettings>;
}


/// Just for testing
fn create_test_enemy(mut commands: Commands, assets: Res<crate::assets::EnemyAssets>) {
    commands.spawn_bundle(EnemySlime::with_assets(&assets.slime));
}
