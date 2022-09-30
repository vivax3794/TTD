//! Game State management

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

/// Is the game paused?
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pause {
    /// Game is paused, logic should be stopped, but be able to resume from where it stopped!
    Paused,
    /// Game is running, logic should be active!
    Running,
}

impl Default for Pause {
    fn default() -> Self {
        Pause::Running
    }
}

// IMPORTANT: IF you add or modify the order of these variant make sure to edit the `next_in_order` function!

/// What part of the turn are we on?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn next_in_order(&self) -> Self {
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
    pub fn go_to_next_state(&self, mut commands: bevy::prelude::Commands) {
        let next_state = iyes_loopless::state::NextState(self.next_in_order());
        commands.insert_resource(next_state);
    }
}
