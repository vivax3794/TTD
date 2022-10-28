//! Tracks and controls turn switching

use std::time::Duration;

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
}

/// Implments auto switching turn state when turn progress is done!
pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        // app.add_loopless_state(TurnState::None)
        app.add_enter_system(crate::MainState::Playing, set_inital_turn_state)
            .add_exit_system(crate::MainState::Playing, remove_turn_state);
        app.add_system(
            make_sure_turn_is_long_enough
                .track_progress()
                .run_in_state(crate::MainState::Playing),
        );

        let turn_order = [
            TurnPart::EnemyTurnStart,
            TurnPart::EnemySpawn,
            TurnPart::EnemyMove,
            TurnPart::EnemyTurnEnd,
            TurnPart::PlayerTurnStart,
            TurnPart::PlayerAction,
            TurnPart::PlayerAttack,
            TurnPart::PlayerTurnEnd,
            TurnPart::EnemyTurnStart,
        ];
        for (&from, &to) in turn_order.iter().zip(turn_order.iter().skip(1)) {
            app.add_plugin(
                ProgressPlugin::new(TurnState::InTurn(from)).continue_to(TurnState::InTurn(to)),
            );
        }
    }
}


/// When we enter gameplay set the inital turn part
fn set_inital_turn_state(mut commands: Commands) {
    commands.insert_resource(NextState(TurnState::InTurn(TurnPart::EnemyTurnStart)));
}

/// Set turn state to None when we are not in gamplay
fn remove_turn_state(mut commands: Commands) {
    commands.insert_resource(NextState(TurnState::None));
}

/// Make turn be at least 100 ms
fn make_sure_turn_is_long_enough(
    global_timer: Res<Time>,
    mut timer: Local<Timer>,
    state: Res<CurrentState<TurnState>>,
) -> Progress {
    if state.is_changed() {
        *timer = Timer::new(Duration::from_millis(50), true);
    }

    timer.tick(global_timer.delta()).finished().into()
}
