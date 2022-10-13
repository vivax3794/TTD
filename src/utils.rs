//! Utility systems like for example renaming entites based on components,

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::Animator;
use iyes_progress::Progress;

/// Create a system that can set the name of entites
pub fn give_entity_name<C: Component>(new_name: String) -> impl FnMut(Query<&mut Name, Added<C>>) {
    move |mut query| {
        query.for_each_mut(|mut name| {
            name.set(new_name.clone());
        });
    }
}

/// Check if all animations of type `T` is done
pub fn is_animation_done<T: Component>(
    query: Query<&Animator<T>, Changed<Animator<T>>>,
) -> Progress {
    let mut done = 0;
    let mut total = 0;

    for animator in query.iter() {
        done += (animator.progress() * 100.) as u32;
        total += 100;
    }

    Progress { done, total }
}

/// Get a field from a entity!
pub fn get_field<'a>(entity: &'a EntityInstance, field_name: &str) -> &'a FieldValue {
    for field in &entity.field_instances {
        if field.identifier == field_name {
            return &field.value;
        }
    }

    panic!("could not find field value with name {}", field_name);
}

/// extracts a value from a pattern, panicing if it doesnt match
macro_rules! extract {
    ($source:expr, $pattern:pat => $result:expr) => {{
        let val = $source;
        match val {
            $pattern => $result,
            _ => panic!("Pattern did not match in extract!({:?}, ...)", val),
        }
    }};
}
