use bevy::prelude::*;

use bevy_tweening::{Animator, AnimatorState};
use iyes_progress::Progress;

/// Create a system that can set the name of entites
pub fn give_entity_name<C: Component>(new_name: String) -> impl FnMut(Query<&mut Name, Added<C>>) {
    move |mut query| {
        query.for_each_mut(|mut name| {
            name.set(new_name.clone());
        })
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

    Progress { total, done }
}
