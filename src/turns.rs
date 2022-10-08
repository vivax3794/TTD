//! Tracks and controls turn switching

use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

/// What part of the turn are we on?
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPart {
    /// Turn start events are handled
    EnemyTurnStart,
    /// Enemies are moved
    EnemyMove,
    /// Potential new enemies are spawned
    EnemySpawn,
    /// Enemies end of turn effects active
    EnemyTurnEnd,
    /// Start of player turn effects activate
    PlayerTurnStart,
    /// Player can take an action.
    /// Like buying a tower.
    PlayerAction,
    /// Player can order towers to do an attack on this turn
    PlayerAttack,
    /// End of turn effect active
    PlayerTurnEnd,
}

/// Are we in a turn or are we switching?

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    /// We are not in gamplay
    None,
    /// We are in the middle of a turn part
    InTurn(TurnPart),
}

/// Implments auto switching turn state when turn progress is done!
pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        // app.add_loopless_state(TurnState::None)
        app.add_enter_system(crate::MainState::Playing, set_inital_turn_state)
            .add_exit_system(crate::MainState::Playing, remove_turn_state);
        app.add_system(set_turn_icon.run_in_state(crate::MainState::Playing));
        app.add_system(make_sure_turn_is_long_enough.track_progress().run_in_state(crate::MainState::Playing));

        use TurnPart::*;
        let turn_order = [
            EnemyTurnStart,
            EnemySpawn,
            EnemyMove,
            EnemyTurnEnd,
            PlayerTurnStart,
            PlayerAction,
            PlayerAttack,
            PlayerTurnEnd,
            EnemyTurnStart,
        ];
        for (&from, &to) in turn_order.iter().zip(turn_order.iter().skip(1)) {
            app.add_plugin(
                ProgressPlugin::new(TurnState::InTurn(from)).continue_to(TurnState::InTurn(to)),
            );
        }
    }
}

/// Mark an entity as the turn icon in the ui
#[derive(Component, Default)]
struct TurnIconMarker;

/// When we enter gameplay set the inital turn part
fn set_inital_turn_state(mut commands: Commands, assets: Res<crate::assets::MiscAssets>) {
    commands.insert_resource(NextState(TurnState::InTurn(TurnPart::EnemyTurnStart)));

    commands
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
}

/// Set turn state to None when we are not in gamplay
fn remove_turn_state(mut commands: Commands) {
    commands.insert_resource(NextState(TurnState::None))
}

/// Set turn icon
fn set_turn_icon(
    current_state: Res<CurrentState<TurnState>>,
    mut query: Query<&mut UiAtlasImage, With<TurnIconMarker>>,
) {
    if current_state.is_changed() {
        let img_index = match current_state.0 {
            TurnState::None => 0,
            TurnState::InTurn(part) => {
                use TurnPart::*;
                match part {
                    EnemyTurnStart => 4,
                    EnemySpawn => 0,
                    EnemyMove => 1,
                    EnemyTurnEnd => 5,
                    PlayerTurnStart => 6,
                    PlayerAction => 2,
                    PlayerAttack => 3,
                    PlayerTurnEnd => 7,
                }
            }
        };

        let mut ui_atlas = query.single_mut();
        ui_atlas.index = img_index;
    }
}

/// Make turn be at least 100 ms
fn make_sure_turn_is_long_enough(global_timer: Res<Time>, mut timer: Local<Timer>, state: Res<CurrentState<TurnState>>) -> Progress {
    if state.is_changed() {
        *timer = Timer::new(Duration::from_millis(300), true);
    }

    timer.tick(global_timer.delta()).finished().into()
}