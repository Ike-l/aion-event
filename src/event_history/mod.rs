use std::{collections::HashMap, time::Instant};

use crate::prelude::EventBuffer;

pub struct EventHistory {
    latest_instant: Instant,
    ticks: HashMap<Instant, EventBuffer>
}

impl EventHistory {
    pub fn read(&self) -> impl Iterator<Item = (&Instant, &EventBuffer)> {
        self.ticks.iter()
    }

    pub fn tick(
        &mut self, 
        instant: Instant,
        event_buffer: EventBuffer,
    ) -> bool {
        match instant.checked_duration_since(self.latest_instant) {
            Some(duration) => {
                // cannot overwrite history!
                if duration.is_zero() {
                    return false
                }

                self.latest_instant = instant;
                self.ticks.insert(instant, event_buffer);

                true
            },
            None => {
                // cannot rewrite history!
                false
            },
        }
    }
}