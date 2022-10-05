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
    fn eye_settings() -> Vec<super::EyeSettings> {
        vec![
            super::EyeSettings {
                offset: Vec2::new(-2.0, 3.0),
                width: 2.0,
                height: 2.0,
            },
            super::EyeSettings {
                offset: Vec2::new(3.0, 3.0),
                width: 2.0,
                height: 2.0
            }
        ]
    }
}