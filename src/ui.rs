//! Handles the user interface.

use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};
use iyes_loopless::prelude::*;

use crate::{TurnPart, TurnState};

/// How much space should the ui have at the bottom of the screen?
pub const BOTTOM_PADDING: f32 = 120.;

/// Ui plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(crate::MainState::Playing, create_ui);
        app.add_system(set_turn_icon.run_in_state(crate::MainState::Playing));
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
                .insert(TurnIconMarker)
                .insert(crate::RemoveOnGameplayExit);
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