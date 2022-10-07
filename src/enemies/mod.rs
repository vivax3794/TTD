//! Controls enemy

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use iyes_loopless::prelude::*;

mod slime;

pub use slime::EnemySlime;

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<EnemyPath>();

        app.add_system(move_eyes_to_cursor.run_in_state(crate::MainState::Playing));

        app.add_system(crate::utils::give_entity_name::<EnemySpawner>("EnemySpanwer".to_owned()));

        // app.add_system(make_eyes_scared.run_in_state(crate::MainState::Playing));
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
    _sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            _m: EnemyMarker,
            _name: Name::new("Enemy"),
            _sprite: SpriteBundle::default(),
        }
    }
}

/// Marker for a eye entity
#[derive(Component)]
struct EyeMarker;

/// Describes where the eyes are, and how big they are.
/// Important these distances are based on the orginal img
#[derive(Component, Default, Debug)]
struct EyeSettings {
    /// Offset from parent center
    offset: Vec2,

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

    /// Enemies should despawn when the gameplay section of the game is over
    _cleanup: crate::RemoveOnGameplayExit,
}

impl Default for EyesBundle {
    fn default() -> Self {
        Self {
            _m: EyeMarker,
            _cleanup: crate::RemoveOnGameplayExit,
            settings: EyeSettings::default(),
            shape: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::splat(1.0),
                    origin: RectangleOrigin::Center,
                },
                DrawMode::Fill(FillMode::color(Color::BLACK)),
                Transform::from_xyz(0.0, 0.0, 11.0),
            ),
        }
    }
}

/// Describes a trait implenting the needed function for creating a enemy
trait EnemyConstruction: Bundle {
    /// Get eye settings for the eye enteties
    fn eye_settings() -> Vec<EyeSettings>;
}

/// Make eyes look at cursor
fn move_eyes_to_cursor(
    cursor_pos: Res<crate::mouse_location::MouseWorldPos>,
    mut query: Query<(&Parent, &mut Transform, &EyeSettings), With<EyeMarker>>,
    query_parent: Query<&GlobalTransform>,
) {
    query.for_each_mut(|(parent, mut trans, settings)| {
        // Get absolute postion of eyes
        // We dont use this entities GlobalTransform since that depends on the local transform... which we are activly setting
        // and would shift with the eyes, calculating it ourself without considering where the eyes are looking means we always
        // get the center
        let abs_pos = {
            let parent_pos = query_parent.get(parent.get()).unwrap();
            parent_pos.translation().truncate()
                + settings.offset * parent_pos.to_scale_rotation_translation().0.truncate()
        };

        // Get the direction the eyes are looking
        // We dont really need to normalize it, but smaller numbers will get us more accurate results.
        let direction = (cursor_pos.0 - abs_pos).normalize();

        // Calculate slop of vector when considring functions f(x) and g(y)
        let x_slope = direction.y / direction.x;
        let y_slope = direction.x / direction.y;

        // calulcate where f(x)/g(y), we use signum to reverse the result when the mouse is on the other side
        // since at that point the slop is negative when the eyes should be up
        let y_offset = x_slope * settings.width / 2.0 * direction.x.signum();
        let x_offset = y_slope * settings.height / 2.0 * direction.y.signum();

        // limit calcualted values to the eyes, this means that the pupil will be stuck in the corner for some sections
        let x_offset = x_offset
            .max(-settings.width / 2.0 + 0.5)
            .min(settings.width / 2.0 - 0.5);
        let y_offset = y_offset
            .max(-settings.height / 2.0 + 0.5)
            .min(settings.height / 2.0 - 0.5);

        // calculate offset from parent center
        let look_offest = Vec2::new(x_offset, y_offset);
        // Set Z-Index to 11, since enemies are on Z = 10.
        trans.translation = (settings.offset + look_offest).extend(11.0);
    })
}

// TODO: Make this work with the new tilemap system
// /// Make pupils smaller when we hover over the enemy
// /// We detect this using the grid position of the parent!
// fn make_eyes_scared(
//     mouse_grid_location: Res<crate::grid::GridMouseLocation>,
//     mut query_eyes: Query<(&Parent, &mut Transform), With<EyeMarker>>,
//     query_parent: Query<&crate::GridLocation>
// ) {
//     query_eyes.for_each_mut(|(parent, mut trans)| {
//         let grid_location = query_parent.get(parent.get()).unwrap();

//         if grid_location.x == mouse_grid_location.0 && grid_location.y == mouse_grid_location.1 {
//             trans.scale = Vec3::new(0.3, 0.3, 1.0);
//         } else {
//             trans.scale = Vec3::splat(1.0);
//         }

//     })
// }

/// The path for enemies to follow!
#[derive(Reflect, Default, Clone, Debug, Component)]
#[reflect(Component)]
struct EnemyPath(Vec<(i32, i32)>);

impl From<EntityInstance> for EnemyPath {
    fn from(instance: EntityInstance) -> Self {
        let mut path = Vec::new();
        for field in instance.field_instances {
            if field.identifier == "Path" {
                if let FieldValue::Points(points) = field.value {
                    path = points
                        .into_iter()
                        .filter_map(|point| point.map(|p| (p.x, p.y)))
                        .collect();
                    break;
                }
            }
        }

        EnemyPath(path)
    }
}

/// Mark the enemy spawners
#[derive(Debug, Default, Component)]
pub struct EnemySpawner;

/// Spawn enemies on a timer
#[derive(Bundle, Debug, LdtkEntity)]
pub struct EnemySpawnerBundle {
    /// Make this entity easier to find in the editor
    _n: Name,

    /// Mark this entity as a enemy spawner
    _m: EnemySpawner,

    /// Give this bundle all the needed components to exsist in the world
    #[bundle]
    _s: SpatialBundle,

    /// The path spawned enemies should take
    #[from_entity_instance]
    path: EnemyPath,

    /// What location in the grid are you on?
    #[grid_coords]
    position: GridCoords
}