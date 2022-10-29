//! Put boats  on enemies that are underwater

use bevy::prelude::*;

use crate::grid_position::GridPosition;
use crate::ldtk_loader::{get_tile_type_at, TileType};

/// Marker for the boat entity
#[derive(Debug, Default, Component, Clone, Copy)]
pub struct BoatMarker;

/// Spawn boat under enemies on water
pub fn spawn_despawn_boats(
    mut commnads: Commands,
    enemy_query: Query<
        (Entity, &Children, &GridPosition),
        (
            With<super::enemy_components::EnemyMarker>,
            Changed<GridPosition>,
        ),
    >,
    boat_query: Query<Entity, With<BoatMarker>>,
    assets: Res<crate::assets::MiscAssets>,
    asset_store: Res<bevy::asset::Assets<bevy_ecs_ldtk::LdtkAsset>>,
    current_level: Res<bevy_ecs_ldtk::LevelSelection>,
) {
    let world_data = asset_store.get(&assets.ldtk_source_file).unwrap();
    let level_data = world_data.get_level(&current_level).unwrap();
    let tilemap = level_data.layer_instances.as_ref().unwrap().last().unwrap();

    for (enemy, children, pos) in enemy_query.iter() {
        let tile_type = get_tile_type_at(tilemap, pos.0);

        dbg!(&tile_type);

        let child = children
            .iter()
            .find(|child| boat_query.get(**child).is_ok());

        dbg!(child);

        // spawn if on water and does not have boat
        if tile_type == TileType::Water && child.is_none() {
            commnads.entity(enemy).add_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: assets.boat.clone_weak(),
                        ..Default::default()
                    })
                    .insert(BoatMarker);
            });
        }
        // despawn if not on water and has boat
        else if tile_type != TileType::Water {
            if let Some(child) = child {
                commnads.entity(*child).despawn();
            }
        }
    }
}
