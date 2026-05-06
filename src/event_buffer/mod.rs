use std::collections::HashSet;

use crate::prelude::Event;

#[derive(Debug, Default)]
pub struct EventBuffer {
    events: HashSet<Event>
}

impl EventBuffer {
    pub fn read(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn new(events: impl Iterator<Item = Event>) -> Self {
        Self { events: events.collect() }
    }

    pub fn extend(&mut self, other: impl Iterator<Item = Event>) {
        self.events.extend(other);
    }

    pub fn insert(&mut self, event: Event) {
        self.events.insert(event);
    }

    pub fn contains(&self, event: &Event) -> bool {
        self.events.contains(event)
    }
}