use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::events::event::{IEvent, UserEvent, SystemEvent};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventCandid {
    User(UserEvent),
    System(SystemEvent),
}

pub trait Candid {
    fn to_event(&self) -> Box<dyn IEvent>;
}

impl Candid for EventCandid {
    fn to_event(&self) -> Box<dyn IEvent> {
        match self {
            EventCandid::User(event) => Box::new(event.clone()),
            EventCandid::System(event) => Box::new(event.clone()),
        }
    }
}
