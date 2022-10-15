//! Core enemy logic and bundles

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::enemy_types::EnemyType;
use crate::{grid_position::GridPosition, utils::get_field};

/// The path for enemies to follow!
#[derive(Reflect, Default, Clone, Debug, Component)]
#[reflect(Component)]
pub struct EnemyPath(pub usize, pub Vec<IVec2>);

/// All enmies will have this components
#[derive(Component, Default)]
pub struct EnemyMarker;

/// Bundle containing everything a enemy will need
#[derive(Bundle)]
pub struct EnemyBundle {
    /// Marker so we know this is a enemy
    pub _m: EnemyMarker,
    /// Make us be able to reconise an enemy!
    pub _name: Name,

    /// Holds dynamic vector art
    #[bundle]
    pub _sprite: SpriteBundle,

    /// Path enemy needs to take
    pub path: EnemyPath,

    /// Location of enemy in grid, updated by move system
    pub grid_location: GridPosition,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            _m: EnemyMarker,
            _name: Name::new("Enemy"),
            _sprite: SpriteBundle::default(),
            path: EnemyPath(0, Vec::new()),
            grid_location: GridPosition::default(),
        }
    }
}

// impl From<EntityInstance> for EnemyPath {
//     fn from(instance: EntityInstance) -> Self {
//         let path = extract!(get_field(&instance, "Path"), FieldValue::Points(points) => points);
//         let path = path.iter().filter_map(|val| *val).collect();
//         EnemyPath(path)
//     }
// }

/// Spawner order of enemies
#[derive(Default, Clone, Debug, Component)]
pub struct EnemyWaves(pub usize, pub Vec<Option<EnemyType>>);

impl From<EntityInstance> for EnemyWaves {
    fn from(instance: EntityInstance) -> Self {
        let waves = extract!(get_field(&instance, "EnemyType"), FieldValue::Enums(vals) => vals);
        let waves = waves
            .iter()
            .map(|value| value.as_ref().map(EnemyType::from))
            .collect();

        EnemyWaves(0, waves)
    }
}

/// Mark the enemy spawners
#[derive(Debug, Default, Component)]
pub struct EnemySpawner;

/// All components of a enemy spawner that dont require custom build logic!
///
/// The path component requires extra world acess, but implementing all the dervied macro does manually is a fucking pain
/// So we derive it for the easy components, then implenet another bundle (with this one as a sub-bundle) and do only the custom logic there!
#[derive(Bundle, Debug, LdtkEntity)]
struct DerviedEnemySpawnerBundle {
    /// Make this entity easier to find in the editor
    pub _n: Name,

    /// Mark this entity as a enemy spawner
    pub _m: EnemySpawner,

    /// Give this bundle all the needed components to exsist in the world
    #[bundle]
    pub _s: SpatialBundle,

    /// Waves to spawn
    #[from_entity_instance]
    pub wave: EnemyWaves,
    // /// What location in the grid are you on?
    // #[from_entity_instance]
    // pub position: GridPosition,
}

/// Spawn enemies on a timer
#[derive(Bundle, Debug)]
pub struct EnemySpawnerBundle {
    /// Core easy to derive components
    #[bundle]
    _core: DerviedEnemySpawnerBundle,

    /// What path should enemies take
    path: EnemyPath,

    /// Location of enemy spawner
    location: GridPosition,
}

// FieldValue points have a filed origin
// So we need to do some manual construction with more world acess
// https://github.com/Trouv/bevy_ecs_ldtk/issues/124
impl LdtkEntity for EnemySpawnerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let path = extract!(get_field(entity_instance, "Path"), FieldValue::Points(path) => path);
        let level_height = layer_instance.c_hei;
        let path = path
            .iter()
            .filter_map(|point| point.map(|point| IVec2::new(point.x, level_height - point.y - 1)))
            .collect();

        Self {
            _core: DerviedEnemySpawnerBundle::bundle_entity(
                entity_instance,
                layer_instance,
                tileset,
                tileset_definition,
                asset_server,
                texture_atlases,
            ),

            path: EnemyPath(0, path),
            location: GridPosition(IVec2::new(
                entity_instance.grid.x,
                level_height - entity_instance.grid.y - 1,
            )),
        }
    }
}
