//! Load the game levels created in `LDtk`

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;

/// Loads an managed `LDtk` level files
pub struct LDtkMangerPlugin;
impl Plugin for LDtkMangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin);
        app.insert_resource(LevelSelection::Index(0));

        app.register_ldtk_entity::<crate::enemies::EnemySpawnerBundle>("EnemySpawner");

        app.add_enter_system(crate::MainState::Playing, setup_tilemap);
    }
}

/// Insert tilemap resources
fn setup_tilemap(mut commands: Commands, assets: Res<crate::assets::MiscAssets>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: assets.ldtk_source_file.clone(),
        ..default()
    });
}
