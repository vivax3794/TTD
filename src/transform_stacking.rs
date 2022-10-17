//! Allow multiple different transform offsets to happen on a single entitiy

// Give up on this idea?

use bevy::prelude::*;
use bevy_tweening::Lens;

use derive_more::{Index, IndexMut};

/// Plugin for calculating stuff!
pub struct StackTransformPlugin;

impl Plugin for StackTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(calculate_stacked_transforms);
        app.add_system(add_index_references);
    }
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!! WARNING !!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// ! This code is gonna be really bad!
// ! but I dont think I can do anything better really :(
// ! Please dont burn me on the cross :/

// ! OH MY FUCKING GOD THIS IS GONNA BE A FUCKING MESS :)
/// Holds a number in its type
#[derive(Component, Default)]
pub struct StackTransformReference<const N: usize>(Transform);

/// Apply lens to a specific index of a `StackedTransforms`
pub struct StackTransformLens<L: Lens<Transform>, const N: usize>(pub L);

impl<const N: usize, L: Lens<Transform>> Lens<StackTransformReference<N>> for StackTransformLens<L, N> {
    fn lerp(&mut self, target: &mut StackTransformReference<N>, ratio: f32) {
        let mut current = target.0;
        self.0.lerp(&mut current, ratio);
    }
}


/// All transforms in in this vector will be calculated into the final transform
/// ! Only supports `translation` and `scale`
#[derive(Component, Debug, Clone, Index, IndexMut)]
pub struct StackedTransforms(Vec<Transform>);

impl StackedTransforms {
    /// Create a new stacked transform wit hthe idicated number of elements
    #[must_use = "this is a new function, ofc you need to use the result"]
    pub fn new(len: usize) -> Self {
        Self(vec![Transform::default(); len])
    }
}

/// Add index references to items
fn add_index_references(mut commands: Commands, query: Query<(Entity, &StackedTransforms), Added<StackedTransforms>>) {
    for (entity, stack) in query.iter() {
        let mut entity = commands.entity(entity);

        for index in 0..stack.0.len() {
            // .... we have a dynamic number that we need to elevate to the type system
            // but ofc type system lives at compile time so this is impossible ....
            // WELL FUCK 
            // ..................... OR we could commit every sin known to man!
            // !!!!!!!!!!!!!!!!!!! CURSED CODE TIME !!!!!!!!!!!!!!!!!!!
            match index {
                0 => entity.insert(StackTransformReference::<0>(Transform::default())),
                1 => entity.insert(StackTransformReference::<1>(Transform::default())),
                2 => entity.insert(StackTransformReference::<2>(Transform::default())),
                3 => entity.insert(StackTransformReference::<3>(Transform::default())),
                4 => entity.insert(StackTransformReference::<4>(Transform::default())),
                5 => entity.insert(StackTransformReference::<5>(Transform::default())),
                _ => panic!("well I didnt think we needed this many")
            };
        }
    }
}

// We dont worry about removing them again... because well that is actually unlikely :D


/// Update actual 

/// Calculate `Transform` from `StackedTransforms`
fn calculate_stacked_transforms(mut query: Query<(&mut Transform, &StackedTransforms)>) {
    query.for_each_mut(|(mut trans, stacked)| {
        *trans = stacked.0.clone().into_iter().reduce(|current, before| {
            Transform {
                translation: current.translation + before.translation,
                scale: current.scale * before.scale,
                rotation: current.rotation * before.rotation
            }
        }).unwrap();
    });
}