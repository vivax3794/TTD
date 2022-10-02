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

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum TurnState {
    InTurn(TurnPart),
    Switching(TurnPart, TurnPart),
}

/// Implments auto switching turn state when turn progress is done!
pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(TurnState::InTurn(TurnPart::EnemyTurnStart));

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
            app.add_plugin(ProgressPlugin::new(TurnState::InTurn(from)).continue_to(TurnState::Switching(from, to)));
        }
    }
}

// fn is_switching()