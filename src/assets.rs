/// Loads game assets
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

/// Load assets
pub struct AssetLoadingPlugin;

/// How much should we scale stuff up?
pub const ASSET_SCALE_UP: f32 = 1.0;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Prevents blurry sprites

        app.add_loading_state(
            LoadingState::new(crate::MainState::LoadingAssets)
                .with_collection::<GameAssets>()
                .with_collection::<MiscAssets>()
                .continue_to_state(crate::MainState::Playing)
        );
        // TODO: go to main menu
        // app.add_plugin(ProgressPlugin::new(crate::MainState::LoadingAssets).continue_to(crate::MainState::Playing));
    }
}

/// Slime is best enemy
#[derive(AssetCollection, Debug)]
pub struct GameAssets {
    /// Slime Enemy
    #[asset(path = "Slime.png")]
    pub slime: Handle<Image>,
}

/// misc assets
#[derive(AssetCollection, Debug)]
pub struct MiscAssets {
    /// Tilemap
    #[asset(path = "Level.ldtk")]
    pub ldtk_source_file: Handle<bevy_ecs_ldtk::prelude::LdtkAsset>
}
