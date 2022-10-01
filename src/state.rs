//! Game State management

use bevy::prelude::*;
use iyes_loopless::{prelude::AppLooplessStateExt, state::NextState};


// IMPORTANT: IF you add or modify the order of these variant make sure to edit the `next_in_order` function!

/// What part of the turn are we on?
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
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

impl Turn {
    /// Get the next enum variant in the turn order
    pub fn next_in_order(self) -> Self {
        match self {
            Self::EnemyTurnStart => Self::EnemyMove,
            Self::EnemyMove => Self::EnemySpawn,
            Self::EnemySpawn => Self::EnemyTurnEnd,
            Self::EnemyTurnEnd => Self::PlayerTurnStart,
            Self::PlayerTurnStart => Self::PlayerAction,
            Self::PlayerAction => Self::PlayerAttack,
            Self::PlayerAttack => Self::PlayerTurnEnd,
            Self::PlayerTurnEnd => Self::EnemyTurnStart,
        }
    }

    // This is the only place in the file that deals with bevy and iyes_loopless directly, we don't need to pull in an import for that
    /// Insert a resource to move to the next state!
    pub fn go_to_next_state(self, mut commands: bevy::prelude::Commands) {
        let next_state = iyes_loopless::state::NextState(self.next_in_order());
        commands.insert_resource(next_state);
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
/// Main game state
pub enum Main {
    /// A game is running
    Playing,
    /// We are on the main menu
    MainMenu,
}

impl Default for Main {
    fn default() -> Self {
        // TODO: Make it start at the main menu
        Main::Playing
    }
}

/// Handle if the game is currently paused
#[derive(Default)]
pub struct IsPaused(pub bool);


/// Manage init state
pub struct StatePlugin;
impl Plugin for StatePlugin {
   fn build(&self, app: &mut App) {
       app.add_loopless_state(Main::default());
       app.add_enter_system(Main::Playing, enter_game_state);
       app.add_exit_system(Main::Playing, leave_game_state);
    } 
}

/// Is game paused?
pub fn is_paused(paused: Res<IsPaused>) -> bool {
    paused.0
}

/// Setup resources for playing state
fn enter_game_state(mut commands: Commands) {
    commands.insert_resource(IsPaused::default());
    // Make sure turn state is correct.
    commands.insert_resource(NextState(Turn::EnemyTurnStart));
}

/// Entity 
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct RemoveOnGameplayExit;

/// Remove entities marked as gameplay only when we exit the gameplay state
fn leave_game_state(mut commands: Commands, query: Query<Entity, With<RemoveOnGameplayExit>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
    commands.remove_resource::<IsPaused>()
}