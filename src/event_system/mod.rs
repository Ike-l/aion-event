use std::sync::Arc;

use aion_program::prelude::ProgramRegistry;

use crate::prelude::{EventBuffer, EventHistory};

pub trait EventSystem {
    fn execute(
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        event_history: &EventHistory,
    ) -> EventBuffer;
}