//! Enemy logic

mod enemy_core;
mod enemy_types;
mod enemy_eyes;

pub use enemy_core::EnemySpawnerBundle;

use crate::{TurnPart, TurnState};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<enemy_core::EnemyPath>();

        app.add_system(enemy_eyes::move_eyes_to_cursor.run_in_state(crate::MainState::Playing));

        app.add_system(crate::utils::give_entity_name::<enemy_core::EnemySpawner>(
            "EnemySpanwer".to_owned(),
        ));
        app.add_system(
            crate::utils::is_animation_done::<Transform>
                .track_progress()
                .run_in_state(crate::MainState::Playing),
        );

        // app.add_system(make_eyes_scared.run_in_state(crate::MainState::Playing));

        // TURN SYSTEMS
        app.add_enter_system(TurnState::InTurn(TurnPart::EnemySpawn), enemy_core::spawn_enemies);
        app.add_enter_system(TurnState::InTurn(TurnPart::EnemyMove), enemy_core::move_enemies);
    }
}