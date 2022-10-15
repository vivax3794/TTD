//! Enemy logic

mod enemy_components;
mod enemy_eyes;
mod enemy_systems;
mod enemy_types;

pub use enemy_components::EnemySpawnerBundle;

use crate::{TurnPart, TurnState};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

/// Enemy plugin
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<enemy_components::EnemyPath>();

        app.add_system(enemy_eyes::move_eyes_to_cursor.run_in_state(crate::MainState::Playing));

        app.add_system(crate::utils::give_entity_name::<
            enemy_components::EnemySpawner,
        >("EnemySpanwer".to_owned()));
        app.add_system(
            crate::utils::is_animation_done::<Transform>
                .track_progress()
                .run_in_state(crate::MainState::Playing),
        );

        // TURN SYSTEMS
        app.add_enter_system(
            TurnState::InTurn(TurnPart::EnemySpawn),
            enemy_systems::spawn_enemies,
        );
        app.add_enter_system(
            TurnState::InTurn(TurnPart::EnemyMove),
            enemy_systems::move_enemies,
        );
    }
}
