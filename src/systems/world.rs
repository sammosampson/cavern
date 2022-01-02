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
