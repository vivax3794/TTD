//! Define settings and beheaviour of different enemy types

use bevy::prelude::*;
use super::enemy_eyes::EyeSettings;

/// Enemy Types
#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    /// Most basic enemy, the slime!
    Slime,
}

impl From<&str> for EnemyType {
    fn from(name: &str) -> Self {
        match name {
            "Slime" => Self::Slime,
            // We panic because the strings based in should always be valid
            // So a unknown string means there is a type somewhere
            _ => panic!("unknown enemy variant: {}", name),
        }
    }
}

impl EnemyType {
    /// how to construct eyes
    pub fn eye_settings(self) -> Vec<EyeSettings> {
        match self {
            Self::Slime => vec![
                EyeSettings {
                    offset: Vec2::new(-1.0, 1.0),
                    width: 2.0,
                    height: 2.0,
                },
                EyeSettings {
                    offset: Vec2::new(2.0, 1.0),
                    width: 2.0,
                    height: 2.0,
                },
            ],
        }
    }

    /// what img asset should be used for this enemy?
    pub fn enemy_asset(self, assets: &crate::assets::EnemyAssets) -> Handle<Image> {
        match self {
            Self::Slime => assets.slime.clone_weak()
        }
    }
}
