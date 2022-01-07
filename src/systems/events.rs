use crate::prelude::*;

#[system(simple)]
pub fn proliferate_system_events(
    #[resource] event_channel: &mut SystemEventChannel,
    buffer: &mut CommandBuffer,
) {
    for event in event_channel.read_events() {
        buffer.push((event.clone(), ));
    }
}

#[system(for_each)]
#[filter(component::<SystemEvent>())]
pub fn destroy_system_events(
    entity: &Entity, 
    buffer: &mut CommandBuffer,
) {
    buffer.remove(*entity);
} 