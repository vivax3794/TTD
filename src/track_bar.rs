//! A track bar shows a amount of the total

use bevy::prelude::*;

/// Run the track bar systems
pub struct TrackbarPlugin;

impl Plugin for TrackbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_progress_amount);
    }
}

/// Holds trackbar settings
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct TrackbarSettings {
    /// Total amount
    pub total: usize,

    /// Filled color
    pub filled_color: Color,

    /// Not Filled color
    pub background_color: Color,
}

/// Current progress of componet
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct TrackbarProgess(pub usize);

/// Marker for the filled part of the tracker
#[derive(Debug, Component, Default, Clone, Copy)]
struct FilledMarker;

/// All components used by a track bar
#[derive(Bundle, Default)]
pub struct TrackbarBundle {
    /// Settings
    pub settings: TrackbarSettings,
    /// Progress
    pub progress: TrackbarProgess,
    /// Give us an position
    #[bundle]
    pub ui_components: NodeBundle,
}

/// When a trackbar is created also spawn the children
fn spawn_children(
    mut commands: Commands,
    query: Query<(Entity, &TrackbarSettings), Added<TrackbarSettings>>,
) {
    for (entity, settings) in query.iter() {
        commands.entity(entity).add_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                color: settings.background_color.into(),
                ..Default::default()
            });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.), Val::Percent(100.)),
                        ..Default::default()
                    },
                    color: settings.filled_color.into(),
                    ..Default::default()
                })
                .insert(FilledMarker);
        });
    }
}

/// Set progress amount
fn set_progress_amount(
    mut query: Query<(&mut Style, &Parent), With<FilledMarker>>,
    p_query: Query<(&TrackbarProgess, &TrackbarSettings)>,
) {
    for (mut style, parent) in query.iter_mut() {
        let (progress, settings) = p_query.get(parent.get()).unwrap();

        let procent_done = progress.0 as f32 / settings.total as f32;

        style.size.width = Val::Percent(procent_done * 100.);
    }
}
