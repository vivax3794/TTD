//! Enemy sytems control enemy behaviour!

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};

use crate::grid_position::GridPosition;

use super::enemy_components::{EnemyBundle, EnemyMarker, EnemyPath, EnemySpawner, EnemyWaves};
use super::enemy_eyes::EyesBundle;

/// Spawn enemies when it is time
pub fn spawn_enemies(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut EnemyWaves, &EnemyPath, &GridPosition), With<EnemySpawner>>,
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
    });
}

/// Move enemies to next location
pub fn move_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut EnemyPath, &mut GridPosition), With<EnemyMarker>>,
) {
    for (entity, pos, mut path, mut grid_loc) in query.iter_mut() {
        if path.0 != path.1.len() {
            let next_point = path.1[path.0];
            let direction = (next_point - IVec2::from(*grid_loc)).clamp(IVec2::NEG_ONE, IVec2::ONE);
            let world_pos_direction = Vec2::new(direction.x as f32 * 16., direction.y as f32 * 16.);

            let tween = Tween::new(
                EaseFunction::ExponentialInOut,
                TweeningType::Once,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: pos.translation,
                    end: pos.translation + world_pos_direction.extend(0.),
                },
            );

            commands.entity(entity).insert(Animator::new(tween));
            grid_loc.0 += direction;

            // Move to next point
            if grid_loc.0 == next_point {
                path.0 += 1;
            }
        }
    }
}
