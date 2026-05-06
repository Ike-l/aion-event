pub mod event_system;
pub mod event;
pub mod event_buffer;

pub mod prelude {
    pub use super::{
        event::{
            Event,
            event_id::{
                EventId
            }
        },
        event_system::{
            EventSystem
        },
        event_buffer::{
            EventBuffer
        }
    };
}