//! Player control systems

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

/// Player stuff
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(crate::MainState::Playing, reset_health);
    }
}

/// Health the player got left
pub struct PlayerHealth(pub u8);

/// Reset health back to default when we enter gameplay
fn reset_health(mut commands: Commands) {
    commands.insert_resource(PlayerHealth(10));
}

