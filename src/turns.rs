//! Tracks and controls turn switching

use bevy::prelude::*;
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
    /// We are switching to a new turn
    Switching(TurnPart),
}

/// Implments auto switching turn state when turn progress is done!
pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        // app.add_loopless_state(TurnState::None)
        app.add_enter_system(crate::MainState::Playing, set_inital_turn_state)
            .add_exit_system(crate::MainState::Playing, remove_turn_state);

        use TurnPart::*;
        let turn_order = [
            EnemyTurnStart,
            EnemyMove,
            EnemySpawn,
            EnemyTurnEnd,
            PlayerTurnStart,
            PlayerAction,
            PlayerAttack,
            PlayerTurnEnd,
            EnemyTurnStart,
        ];
        for (&from, &to) in turn_order.iter().zip(turn_order.iter().skip(1)) {
            app.add_plugin(
                ProgressPlugin::new(TurnState::InTurn(from)).continue_to(TurnState::Switching(to)),
            );
            app.add_plugin(
                ProgressPlugin::new(TurnState::Switching(to)).continue_to(TurnState::InTurn(to)),
            );
        }
    }
}

/// When we enter gameplay set the inital turn part
fn set_inital_turn_state(mut commands: Commands) {
    commands.insert_resource(NextState(TurnState::InTurn(TurnPart::EnemyTurnStart)));

    commands.spawn_bundle(TextBundle::from_section(
        "hello",
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

/// Set turn state to None when we are not in gamplay
fn remove_turn_state(mut commands: Commands) {
    commands.insert_resource(NextState(TurnState::None))
}
