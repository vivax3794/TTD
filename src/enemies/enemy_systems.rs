//! Enemy sytems control enemy behaviour!

use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};

use crate::grid_position::GridPosition;

use super::enemy_components::{
    EnemyBundle, EnemyHealth, EnemyMarker, EnemyPath, EnemySpawner, EnemyWaves,
};
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
            let health_bar_settings = crate::track_bar::TrackbarSettings {
                total: enemy_type.enemy_health() as usize,
                width: 15.,
                filled_color: Color::rgb(0.8, 0., 0.),
                background_color: Color::rgb(0., 0., 0.),
            };

            commands
                // Main enemy attributes
                .spawn_bundle(EnemyBundle {
                    _sprite: SpriteBundle {
                        texture: enemy_type.enemy_asset(&assets),
                        transform: Transform::from_translation(
                            pos.translation.truncate().extend(10.),
                        ),
                        ..default()
                    },
                    health: EnemyHealth(enemy_type.enemy_health()),
                    path: path.clone(),
                    grid_location: *grid_pos,
                    enemy_type,
                    ..default()
                })
                // Spawn eyes
                .with_children(|parent| {
                    for settings in enemy_type.eye_settings() {
                        parent.spawn_bundle(EyesBundle::from_settings(settings));
                    }
                })
                // Spawn enemy health sub entities
                .with_children(|parent| {
                    parent
                        .spawn_bundle(crate::track_bar::TrackbarBundle {
                            settings: health_bar_settings,
                            position: SpatialBundle {
                                transform: Transform {
                                    translation: Vec3::new(0., -10., 1.),
                                    scale: Vec3::new(1., 3., 1.),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .add_children(|parent| {
                            crate::track_bar::TrackbarBundle::create_children(
                                &health_bar_settings,
                                parent,
                            );
                        });
                })
                // Create spawn anumation
                .insert(Animator::new(Tween::new(
                    EaseFunction::BounceOut,
                    TweeningType::Once,
                    Duration::from_millis(1000),
                    TransformScaleLens {
                        start: Vec3::ZERO,

                        // Scale from 16 px to 10 px?
                        // 16 * X = 10 => X = 10 / 16
                        end: Vec3::new(10. / 16., 10. / 16., 1.),
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
            let next_target_point = path.1[path.0];
            let direction =
                (next_target_point - IVec2::from(*grid_loc)).clamp(IVec2::NEG_ONE, IVec2::ONE);

            let next_point = grid_loc.0 + direction;

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
            grid_loc.0 = next_point;

            // Move to next point
            if grid_loc.0 == next_target_point {
                path.0 += 1;
            }
        }
    }
}

/// Set healthbar progress to the current health
pub fn update_healthbar(
    query: Query<(&EnemyHealth, &Children), Changed<EnemyHealth>>,
    mut progress_query: Query<&mut crate::track_bar::TrackbarProgess>,
) {
    for (health, children) in query.iter() {
        for child in children.iter() {
            if let Ok(mut progress) = progress_query.get_mut(*child) {
                progress.0 = health.0 as usize;
            }
        }
    }
}

// TODO: Since we are going with the health bar idea
// TODO: we are gonna have to wait until I implement that before we can work on stacking
