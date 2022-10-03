/// Loads game assets
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

pub struct AssetLoadingPlugin;

/// How much should we scale stuff up?
pub const ASSET_SCALE_UP: f32 = 5.0;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Prevents blurry sprites

        app.add_loading_state(
            LoadingState::new(crate::MainState::LoadingAssets)
                .with_collection::<EnemyAssets>()
                .continue_to_state(crate::MainState::Playing)
        );
        // TODO: go to main menu
        // app.add_plugin(ProgressPlugin::new(crate::MainState::LoadingAssets).continue_to(crate::MainState::Playing));
    }
}

/// Slime is best enemy
#[derive(AssetCollection, Debug)]
pub struct EnemyAssets {
    /// Sprite sheet for slime
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 2, rows =1))]
    #[asset(path = "Enemy/Slime.png")]
    pub slime: Handle<TextureAtlas>,
}
