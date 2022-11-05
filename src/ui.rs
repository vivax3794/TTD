//! Handles the user interface.

use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};
use iyes_loopless::prelude::*;

use crate::{track_bar::TrackbarBundle, TurnPart, TurnState};

/// How much space should the ui have at the bottom of the screen?
pub const BOTTOM_PADDING: f32 = 120.;

/// Ui plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(crate::MainState::Playing, create_ui);
        app.add_system(set_turn_icon.run_in_state(crate::MainState::Playing));
        app.add_system(create_health_bar);
        app.add_system(set_health_bar_progress);
        app.add_system(move_health_bar);
    }
}

/// Mark an entity as the turn icon in the ui
#[derive(Component, Default)]
struct TurnIconMarker;

/// Spawn a light gray rectangle at the bottom of the screen to cover the bottom padding
fn create_ui(mut commands: Commands, assets: Res<crate::assets::MiscAssets>) {
    commands
        // Bottom rectangle
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(BOTTOM_PADDING)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..Default::default()
        })
        .insert(crate::RemoveOnGameplayExit)
        .add_children(|parent| {
            // Turn icons
            parent
                .spawn_bundle(AtlasImageBundle {
                    atlas_image: UiAtlasImage {
                        atlas: assets.turn_icons.clone_weak(),
                        index: 0,
                    },
                    style: Style {
                        size: Size::new(Val::Px(16. * 5.), Val::Px(16. * 5.)),
                        margin: UiRect {
                            left: Val::Px(16.),
                            bottom: Val::Px(16.),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(TurnIconMarker);
        });

    // We are gonna fake the other UI elements using world space (since our camera doesnt move)
}

/// Set turn icon
fn set_turn_icon(
    current_state: Res<CurrentState<TurnState>>,
    mut query: Query<&mut UiAtlasImage, With<TurnIconMarker>>,
) {
    if current_state.is_changed() {
        let img_index = match current_state.0 {
            TurnState::None => 0,
            TurnState::InTurn(part) => match part {
                TurnPart::EnemyTurnStart => 4,
                TurnPart::EnemySpawn => 0,
                TurnPart::EnemyMove => 1,
                TurnPart::EnemyTurnEnd => 5,
                TurnPart::PlayerTurnStart => 6,
                TurnPart::PlayerAction => 2,
                TurnPart::PlayerAttack => 3,
                TurnPart::PlayerTurnEnd => 7,
            },
        };

        let mut ui_atlas = query.single_mut();
        ui_atlas.index = img_index;
    }
}

///  Tells the healthbar what enemy to follow
#[derive(Component, Clone, Debug)]
struct EnemyHealthBarFollow(Entity);

/// create a healthbar for now enemies
fn create_health_bar(
    mut commands: Commands,
    query: Query<(Entity, &crate::enemies::EnemyHealth), Added<crate::enemies::EnemyMarker>>,
) {
    for (entity, health) in query.iter() {
        commands
            .spawn_bundle(TrackbarBundle {
                ui_components: NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(40.), Val::Px(10.)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(500.),
                            left: Val::Px(100.),
                            ..Default::default()
                        },
                        ..default()
                    },
                    ..default()
                },
                settings: crate::track_bar::TrackbarSettings {
                    total: health.0 as usize,
                    filled_color: Color::rgb(1., 0., 0.),
                    background_color: Color::rgb(0., 0., 0.),
                },
                ..default()
            })
            .insert(EnemyHealthBarFollow(entity));
    }
}

/// Set the healthbar progress to the health of the enemy
fn set_health_bar_progress(
    health_query: Query<&crate::enemies::EnemyHealth>,
    mut query: Query<(
        &mut crate::track_bar::TrackbarProgess,
        &EnemyHealthBarFollow,
    )>,
) {
    for (mut progress, follow) in query.iter_mut() {
        if let Ok(health) = health_query.get(follow.0) {
            progress.0 = health.0 as usize;
        }
    }
}

/// Move healthbar to follow the enemy
/// We need to compansite for world and camera matrix to get the correct position
fn move_health_bar(
    mut health_query: Query<(&mut Style, &EnemyHealthBarFollow)>,
    enemy_query: Query<&GlobalTransform, With<crate::enemies::EnemyMarker>>,
    wnds: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
) {
    let window = wnds.get_primary().unwrap();
    let (camera, camera_transform) = camera_query.single();

    for (mut style, follow) in health_query.iter_mut() {
        if let Ok(enemy_transform) = enemy_query.get(follow.0) {
            let world_pos = enemy_transform.translation();

            let matrix = (camera_transform.compute_matrix() * camera.projection_matrix().inverse()).inverse();
            let screen_pos_trunc = matrix.project_point3(world_pos);

            let windows_size = Vec2::new(window.width(), window.height());
            let screen_pos = (screen_pos_trunc.truncate() + Vec2::ONE) / 2. * windows_size;

            style.position.left = Val::Px(screen_pos.x);
            style.position.bottom = Val::Px(screen_pos.y);
        }
    }
}
