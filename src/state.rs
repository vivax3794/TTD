//! Game State management

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
/// Main game state
pub enum Main {
    /// Loading game assets!
    LoadingAssets,
    /// A game is running
    Playing,
    /// We are on the main menu
    MainMenu,
}


/// Manage init state
pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(Main::Playing, leave_game_state);
    }
}

/// Entity
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub struct RemoveOnGameplayExit;

/// Remove entities marked as gameplay only when we exit the gameplay state
fn leave_game_state(mut commands: Commands, query: Query<Entity, With<RemoveOnGameplayExit>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}
