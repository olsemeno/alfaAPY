use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::enums::{UserEventType, UserEventDetails, SystemEventType, SystemEventDetails};
use crate::events::event_candid::EventCandid;
use crate::types::types::EventResponse;

pub trait IEvent {
    fn get_id(&self) -> u64;
    fn get_timestamp(&self) -> u64;
    fn to_candid(&self) -> EventCandid;
    fn to_response(&self) -> EventResponse;
}

pub trait IUserEvent: IEvent {
    fn get_user(&self) -> Principal;
    fn get_details(&self) -> UserEventDetails;
}

pub trait ISystemEvent: IEvent {
    fn get_details(&self) -> SystemEventDetails;
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserEvent {
    pub id: u64,
    pub timestamp: u64,
    pub event_type: UserEventType,
    pub details: UserEventDetails,
    pub user: Principal,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SystemEvent {
    pub id: u64,
    pub timestamp: u64,
    pub event_type: SystemEventType,
    pub details: SystemEventDetails,
}

// UserEvent implementation

impl UserEvent {
    pub fn new(id: u64, event_type: UserEventType, details: UserEventDetails, timestamp: u64, user: Principal) -> Self {
        Self {
            id,
            timestamp,
            event_type,
            details,
            user,
        }
    }
}

impl IEvent for UserEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn to_candid(&self) -> EventCandid {
        EventCandid::User(self.clone())
    }

    fn to_response(&self) -> EventResponse {
        EventResponse::User(self.clone())
    }
}

impl IUserEvent for UserEvent {
    fn get_user(&self) -> Principal {
        self.user
    }

    fn get_details(&self) -> UserEventDetails {
        self.details.clone()
    }
}

// SystemEvent implementation

impl SystemEvent {
    pub fn new(id: u64, event_type: SystemEventType, details: SystemEventDetails, timestamp: u64) -> Self {
        Self {
            id,
            timestamp,
            event_type,
            details,
        }
    }
}

impl IEvent for SystemEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn to_candid(&self) -> EventCandid {
        EventCandid::System(self.clone())
    }

    fn to_response(&self) -> EventResponse {
        EventResponse::System(self.clone())
    }
}

impl ISystemEvent for SystemEvent {
    fn get_details(&self) -> SystemEventDetails {
        self.details.clone()
    }
}
