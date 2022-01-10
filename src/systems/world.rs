use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Remove>())]
pub fn remove_entity(
    entity: &Entity,
    id: &WorldEntityId,
    buffer: &mut CommandBuffer,
) {
    println!("remove entity {:?}", id);
    buffer.remove(*entity);
}

#[system(for_each)]
#[filter(!component::<Added>())]
pub fn mark_added_entity(
    entity: &Entity,
    id: &WorldEntityId,
    buffer: &mut CommandBuffer,
) {
    println!("added entity {:?}", id);
    buffer.add_component(*entity, Added);
}