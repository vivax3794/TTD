//! Loads game assets

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Load assets
pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Prevents blurry sprites

        app.add_loading_state(
            LoadingState::new(crate::MainState::LoadingAssets)
                .with_collection::<EnemyAssets>()
                .with_collection::<MiscAssets>()
                .continue_to_state(crate::MainState::Playing),
        );
        // TODO: go to main menu
        // app.add_plugin(ProgressPlugin::new(crate::MainState::LoadingAssets).continue_to(crate::MainState::Playing));
    }
}

/// Assets for enemies
#[derive(AssetCollection, Debug)]
pub struct EnemyAssets {
    /// Slime Enemy
    #[asset(path = "Enemies/Slime.png")]
    pub slime: Handle<Image>,

    /// The best sponge
    #[asset(path = "Enemies/Sponge.png")]
    pub sponge: Handle<Image>,
}

/// misc assets
#[derive(AssetCollection, Debug)]
pub struct MiscAssets {
    /// Tilemap
    #[asset(path = "Level.ldtk")]
    pub ldtk_source_file: Handle<bevy_ecs_ldtk::prelude::LdtkAsset>,

    /// Font file
    #[asset(path = "Font.ttf")]
    pub font: Handle<Font>,

    /// Turn icons
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 2))]
    #[asset(path = "TurnIcons.png")]
    pub turn_icons: Handle<TextureAtlas>,

    /// Boat texture
    #[asset(path = "Boat.png")]
    pub boat: Handle<Image>,
    /// Boat flag pole
    #[asset(path = "Boat_Flag.png")]
    pub boat_flag: Handle<Image>,
}
