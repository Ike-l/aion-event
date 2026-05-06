pub mod event_id;

use crate::prelude::EventId;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Event {
    id: EventId
}