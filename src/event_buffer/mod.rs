use crate::prelude::Event;

#[derive(Debug, Default)]
pub struct EventBuffer {
    events: Vec<Event>
}

impl EventBuffer {
    pub fn read(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn extend(&mut self, other: impl Iterator<Item = Event>) {
        self.events.extend(other);
    }

    pub fn contains(&self, event: &Event) -> bool {
        self.events.contains(event)
    }
}