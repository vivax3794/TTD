//! Eyes of the enemy

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

/// Marker for a eye entity
#[derive(Component, Default, Clone, Copy)]
pub struct EyeMarker;

/// Describes where the eyes are, and how big they are.
/// Important these distances are based on the orginal img
#[derive(Component, Default, Debug)]
pub struct EyeSettings {
    /// Offset from parent center
    pub offset: Vec2,

    /// How big is the pupil?
    pub pupil_scale: Vec2,

    /// How big is the eye?
    pub eye_scale: Vec2,
}

/// Eyes follow the mouse cursor!
#[derive(Bundle, Default)]
pub struct EyesBundle {
    /// Marker to identitfy this entity as an eye
    pub _m: EyeMarker,

    /// Contains a basic black square
    #[bundle]
    pub shape: ShapeBundle,

    /// Settings for eye
    pub settings: EyeSettings,

    /// Enemies should despawn when the gameplay section of the game is over
    pub _cleanup: crate::RemoveOnGameplayExit,
}

impl EyesBundle {
    /// Create a `EyeBundle` based on the settings,
    /// you cant just manually pass in `settings` as the `shape` also depends on the inital settings
    pub fn from_settings(settings: EyeSettings) -> Self {
        Self {
            shape: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    origin: RectangleOrigin::Center,
                    extents: settings.pupil_scale,
                },
                DrawMode::Fill(FillMode::color(Color::BLACK)),
                Transform::default(),
            ),
            settings,
            ..default()
        }
    }
}

/// Make eyes look at cursor
pub fn move_eyes_to_cursor(
    cursor_pos: Res<crate::mouse_location::MouseWorldPos>,
    mut query: Query<(&Parent, &mut Transform, &EyeSettings), With<EyeMarker>>,
    query_parent: Query<&GlobalTransform>,
) {
    query.for_each_mut(|(parent, mut trans, settings)| {
        // Get absolute postion of eyes
        // We dont use this entities GlobalTransform since that depends on the local transform... which we are activly setting
        // and would shift with the eyes, calculating it ourself without considering where the eyes are looking means we always
        // get the center
        let abs_pos = {
            let parent_pos = query_parent.get(parent.get()).unwrap();
            parent_pos.translation().truncate()
                + settings.offset * parent_pos.to_scale_rotation_translation().0.truncate()
        };

        // Get the direction from the eyes to the cursor
        // We dont really need to normalize it, but smaller numbers will get us more accurate results.
        let direction = (cursor_pos.0 - abs_pos).normalize();

        // Calculate slop of vector when considring functions f(x) and g(y)
        let x_slope = direction.y / direction.x;
        let y_slope = direction.x / direction.y;

        // calulcate where f(x)/g(y) intersect the edge of the eyes
        // we use `signum` to reverse the result when the mouse is on the other side
        // since at that point the slop is negative when the eyes should be up
        let y_offset = x_slope * settings.eye_scale.x / 2.0 * direction.x.signum();
        let x_offset = y_slope * settings.eye_scale.y / 2.0 * direction.y.signum();

        // limit calcualted values to the eyes,
        // this means that the pupil will be stuck in the corner for some sections
        let x_limit = settings.eye_scale.x / 2. - settings.pupil_scale.x / 2.;
        let y_limit = settings.eye_scale.y / 2. - settings.pupil_scale.y / 2.;
        let x_offset = x_offset.clamp(-x_limit, x_limit);
        let y_offset = y_offset.clamp(-y_limit, y_limit);

        // calculate offset from parent center
        let offset = settings.offset + Vec2::new(x_offset, y_offset);
        // Set Z-Index to 11, since enemies are on Z = 10.
        trans.translation = offset.extend(11.0);
    });
}
