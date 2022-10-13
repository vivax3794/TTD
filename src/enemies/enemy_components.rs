//! Core enemy logic and bundles

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::enemy_types::EnemyType;
use crate::utils::get_field;

/// The path for enemies to follow!
#[derive(Reflect, Default, Clone, Debug, Component)]
#[reflect(Component)]
pub struct EnemyPath(pub Vec<IVec2>);

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
    pub grid_location: GridCoords,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            _m: EnemyMarker,
            _name: Name::new("Enemy"),
            _sprite: SpriteBundle::default(),
            path: EnemyPath(Vec::new()),
            grid_location: GridCoords::default(),
        }
    }
}

impl From<EntityInstance> for EnemyPath {
    fn from(instance: EntityInstance) -> Self {
        let path = extract!(get_field(&instance, "Path"), FieldValue::Points(points) => points);
        let path = path.iter().filter_map(|val| *val).collect();
        EnemyPath(path)
    }
}

/// Spawner order of enemies
#[derive(Default, Clone, Debug, Component)]
pub struct EnemyWaves(pub usize, pub Vec<Option<EnemyType>>);

impl From<EntityInstance> for EnemyWaves {
    fn from(instance: EntityInstance) -> Self {
        let waves = extract!(get_field(&instance, "EnemyType"), FieldValue::Enums(vals) => vals);
        let waves = waves.iter().map(|value| value.as_ref().map(EnemyType::from)).collect();
        
        EnemyWaves(0, waves)
    }
}

/// Mark the enemy spawners
#[derive(Debug, Default, Component)]
pub struct EnemySpawner;

/// Spawn enemies on a timer
#[derive(Bundle, Debug, LdtkEntity)]
pub struct EnemySpawnerBundle {
    /// Make this entity easier to find in the editor
    pub _n: Name,

    /// Mark this entity as a enemy spawner
    pub _m: EnemySpawner,

    /// Give this bundle all the needed components to exsist in the world
    #[bundle]
    pub _s: SpatialBundle,

    /// The path spawned enemies should take
    #[from_entity_instance]
    pub path: EnemyPath,

    /// Waves to spawn
    #[from_entity_instance]
    pub wave: EnemyWaves,

    /// What location in the grid are you on?
    #[grid_coords]
    pub position: GridCoords,
}
