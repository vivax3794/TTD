//! Enemy sytems control enemy behaviour!

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};

use super::enemy_components::{
    EnemyBundle, EnemyMarker, EnemyPath, EnemySpawner, EnemyWaves,
};
use super::enemy_eyes::EyesBundle;

/// Spawn enemies when it is time
pub fn spawn_enemies(
    mut commands: Commands,
    mut query: Query<
        (
            &Transform,
            &mut EnemyWaves,
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
    });
}

/// Move enemies to next location
pub fn move_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut EnemyPath, &mut GridCoords), With<EnemyMarker>>,
) {
    for (entity, pos, mut path, mut grid_loc) in query.iter_mut() {
        if !path.0.is_empty() {
            let pos = pos.translation;

            let next_point = path.0.last().unwrap();

            let direction = ((next_point.x - grid_loc.x), (next_point.y - grid_loc.y));

            dbg!(*grid_loc, next_point, direction);

            let movement_vector =
                Vec2::new(direction.0 as f32, direction.1 as f32).normalize() * 16.;
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

            if grid_loc.x == next_point.x && grid_loc.y == next_point.y {
                path.0.pop().unwrap();
            }
        }
    }
}
