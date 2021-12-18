use crate::prelude::*;

#[system(for_each)]
pub fn exit_if_requested(
    event: &SystemEvent, 
    #[resource] exit_state_notifier: &mut ExitStateNotifier,
) {
    match event {
        SystemEvent::CloseRequested => exit_state_notifier.should_exit = true,
        _ => {}
    }
}