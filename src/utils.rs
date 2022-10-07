use bevy::prelude::*;

/// Create a system that can set the name of entites
pub fn give_entity_name<C: Component>(new_name: String) -> impl FnMut(Query<&mut Name, Added<C>>) {
    move |mut query| query.for_each_mut(|mut name| {
        name.set(new_name.clone());
    })
}