//! Define settings and beheaviour of different enemy types

use super::enemy_eyes::EyeSettings;
use bevy::prelude::*;

/// Enemy Types
#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    /// Most basic enemy, the slime!
    Slime,
}

// NOTE: should we be using `TryFrom` instead since we can clearly fail?
// But on the other hand this conversion should never fail in working code, so we might just want to leave the panic in?
// Since a invalid value will mean there is a error in our program somewhere!

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

impl From<String> for EnemyType {
    fn from(name: String) -> Self {
        let reference: &str = name.as_ref();
        reference.into()
    }
}

impl From<&String> for EnemyType {
    fn from(name: &String) -> Self {
        let reference: &str = name.as_ref();
        reference.into()
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
            Self::Slime => assets.slime.clone_weak(),
        }
    }
}
