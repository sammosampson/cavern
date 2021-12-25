use crate::prelude::*;

#[system(for_each)]
#[filter(!component::<Animation>())]
#[filter(component::<Effect>())]
pub fn remove_dead_effects(
    entity: &Entity,
    buffer: &mut CommandBuffer
) {
    remove_entity(buffer, *entity);
}