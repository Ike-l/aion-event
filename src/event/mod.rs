pub mod event_id;

use crate::prelude::EventId;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Event {
    id: EventId
}