//! Controls enemy

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

use super::enemy_eyes::{move_eyes_to_cursor, EyesBundle};
use super::enemy_types::EnemyType;

use crate::{TurnPart, TurnState};

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<EnemyPath>();

        app.add_system(move_eyes_to_cursor.run_in_state(crate::MainState::Playing));

        app.add_system(crate::utils::give_entity_name::<EnemySpawner>(
            "EnemySpanwer".to_owned(),
        ));
        app.add_system(
            crate::utils::is_animation_done::<Transform>
                .track_progress()
                .run_in_state(crate::MainState::Playing),
        );

        // app.add_system(make_eyes_scared.run_in_state(crate::MainState::Playing));

        // TURN SYSTEMS
        app.add_enter_system(TurnState::InTurn(TurnPart::EnemySpawn), spawn_enemies);
        app.add_enter_system(TurnState::InTurn(TurnPart::EnemyMove), move_enemies);
    }
}
/// The path for enemies to follow!
#[derive(Reflect, Default, Clone, Debug, Component)]
#[reflect(Component)]
struct EnemyPath(Vec<(i32, i32)>);

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

    /// Path enemy needs to take
    path: EnemyPath,

    grid_location: GridCoords,
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
        let mut path = Vec::new();
        for field in instance.field_instances {
            if field.identifier == "Path" {
                if let FieldValue::Points(points) = field.value {
                    path = points
                        .into_iter()
                        .filter_map(|point| point.map(|p| (p.x, 15 - p.y)))
                        .rev()
                        .collect();
                    break;
                }
            }
        }

        EnemyPath(path)
    }
}

/// Spawner order of enemies
#[derive(Default, Clone, Debug, Component)]
struct EnemyWaveSpecification(usize, Vec<Option<EnemyType>>);

impl From<EntityInstance> for EnemyWaveSpecification {
    fn from(entity: EntityInstance) -> Self {
        let mut waves = Vec::new();

        for field in entity.field_instances {
            if field.identifier == "EnemyType" {
                if let FieldValue::Enums(enums_string) = field.value {
                    waves.extend(
                        enums_string
                            .into_iter()
                            .map(|val| val.map(|val| EnemyType::from(val.as_ref()))),
                    )
                }
            }
        }

        Self(0, waves)
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

    /// Waves to spawn
    #[from_entity_instance]
    wave: EnemyWaveSpecification,

    /// What location in the grid are you on?
    #[grid_coords]
    position: GridCoords,
}

/// Spawn enemies when it is time
fn spawn_enemies(
    mut commands: Commands,
    mut query: Query<
        (
            &Transform,
            &mut EnemyWaveSpecification,
            &EnemyPath,
            &GridCoords,
        ),
        With<EnemySpawner>,
    >,
    assets: Res<crate::assets::EnemyAssets>,
) {
    query.for_each_mut(|(pos, mut waves, path, grid_pos)| {
        if waves.0 >= waves.1.len() {
            // We have hit the end of the wave
            // TODO: win condition or something like that?
            return;
        }

        let current_wave = waves.1[waves.0];
        if let Some(enemy_type) = current_wave {
            commands
                .spawn_bundle(EnemyBundle {
                    _sprite: SpriteBundle {
                        texture: enemy_type.enemy_asset(&assets),
                        transform: Transform::from_translation(
                            pos.translation.truncate().extend(10.),
                        ),
                        ..default()
                    },
                    path: path.clone(),
                    grid_location: *grid_pos,
                    ..default()
                })
                .with_children(|parent| {
                    for settings in enemy_type.eye_settings() {
                        parent.spawn_bundle(EyesBundle {
                            settings,
                            ..default()
                        });
                    }
                })
                .insert(Animator::new(Tween::new(
                    EaseFunction::BounceOut,
                    TweeningType::Once,
                    Duration::from_millis(1000),
                    TransformScaleLens {
                        start: Vec3::ZERO,
                        end: Vec3::ONE,
                    },
                )));
        }
        waves.0 += 1;
    })
}

/// Move enemies to next location
fn move_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut EnemyPath, &mut GridCoords), With<EnemyMarker>>,
) {
    for (entity, pos, mut path, mut grid_loc) in query.iter_mut() {
        if !path.0.is_empty() {
            let pos = pos.translation;

            let next_point = path.0.last().unwrap();

            let direction = (
                (next_point.0 - grid_loc.x),
                (next_point.1 - grid_loc.y),
            );

            // dbg!(*grid_loc, next_point, direction);

            let movement_vector = Vec2::new(direction.0 as f32, direction.1 as f32).normalize() * 16.;
            let next_location = pos + movement_vector.extend(0.0);

            let tween = Tween::new(
                EaseFunction::ExponentialInOut,
                TweeningType::Once,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: pos,
                    end: next_location,
                },
            );

            commands.entity(entity).insert(Animator::new(tween));

            grid_loc.x += direction.0.signum() as i32;
            grid_loc.y += direction.1.signum() as i32;

            // dbg!(*grid_loc, next_point);

            if grid_loc.x == next_point.0 && grid_loc.y == next_point.1 {
                path.0.pop().unwrap();
            }
        }
    }
}
