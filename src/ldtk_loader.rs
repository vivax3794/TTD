//! Load the game levels created in `LDtk`

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;

/// Loads an managed `LDtk` level files
pub struct LDtkMangerPlugin;
impl Plugin for LDtkMangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin);
        app.insert_resource(LevelSelection::Index(4));

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

/// Enums for tile types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    /// No tile set
    None,
    /// Grass tile
    Grass,
    /// Path tile
    Path,
    /// Water tile
    Water,
}

/// Get the int grid tile type from the tileset
pub fn get_tile_type_at(tilemap: &LayerInstance, position: IVec2) -> TileType {
    let index = position.x + position.y * tilemap.c_wid;
    let tile = tilemap.int_grid_csv[index as usize];
    match tile {
        1 => TileType::Grass,
        2 => TileType::Water,
        3 => TileType::Path,
        _ => TileType::None,
    }
}