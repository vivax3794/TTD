//! A track bar shows a amount of the total

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Run the track bar systems
pub struct TrackbarPlugin;

impl Plugin for TrackbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_progress_amopunt);
    }
}

/// Holds trackbar settings
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct TrackbarSettings {
    /// Total amount
    pub total: usize,

    /// How wide should the bar be in relation to the height
    pub width: f32,

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
    pub position: SpatialBundle,
}

impl TrackbarBundle {
    /// Create neeeded child entities
    pub fn create_children(&self, mut parent: ChildBuilder) {
        parent.spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2::new(self.settings.width, 1.),
                origin: RectangleOrigin::TopLeft,
            },
            DrawMode::Fill(FillMode::color(self.settings.background_color)),
            Transform::from_xyz(-self.settings.width / 2., 0., 20.),
        ));
        parent
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::new(1., 1.),
                    origin: RectangleOrigin::TopLeft,
                },
                DrawMode::Fill(FillMode::color(self.settings.filled_color)),
                Transform::from_xyz(-self.settings.width / 2., 0., 20.),
            ))
            .insert(FilledMarker);
    }
}

/// Set progress amount
fn set_progress_amopunt(
    mut query: Query<(&mut Transform, &Parent), With<FilledMarker>>,
    p_query: Query<(&TrackbarProgess, &TrackbarSettings)>,
) {
    for (mut trans, parent) in query.iter_mut() {
        let (progress, settings) = p_query.get(parent.get()).unwrap();

        let procent_done = progress.0 as f32 / settings.total as f32;
        let width = settings.width * procent_done;

        trans.scale.x = width;

    }
}
