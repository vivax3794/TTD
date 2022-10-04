//! Simple Slime enemy

use bevy::prelude::*;

use super::EnemyBundle;

/// A simple slime
#[derive(Bundle, Default)]
pub struct EnemySlime {
    /// Common compnents for enemies
    #[bundle]
    _enemy: EnemyBundle,
}

impl super::EnemyConstruction for EnemySlime {
    fn with_assets(assets: &Handle<TextureAtlas>) -> Self {
        Self {
            _enemy: EnemyBundle {
                _sprite: SpriteSheetBundle {
                    texture_atlas: assets.clone(),
                    transform: Transform::from_scale(Vec3::splat(crate::assets::ASSET_SCALE_UP)),
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }

    fn eye_settings() -> Vec<super::EyeSettings> {
        vec![]
    }
}