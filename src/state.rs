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

/// Handle if the game is currently paused
#[derive(Default)]
pub struct IsPaused(pub bool);

/// Manage init state
pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
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
}

/// Entity
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct RemoveOnGameplayExit;

/// Remove entities marked as gameplay only when we exit the gameplay state
fn leave_game_state(mut commands: Commands, query: Query<Entity, With<RemoveOnGameplayExit>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
    commands.remove_resource::<IsPaused>();
}
