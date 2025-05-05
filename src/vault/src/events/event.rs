use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::enums::{UserEventType, UserEventDetails, SystemEventType, SystemEventDetails, UserEventParams, SystemEventParams};
use crate::events::event_candid::EventCandid;
use crate::types::types::EventResponse;
use crate::repository::events_repo::save_event;

pub trait IEvent {
    fn get_id(&self) -> u64;
    fn get_timestamp(&self) -> u64;
    fn to_candid(&self) -> EventCandid;
    fn to_response(&self) -> EventResponse;
    fn save(&self);
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
            event_type,
            details,
            timestamp,
            user,
        }
    }

    pub fn from_params(id: u64, params: UserEventParams, timestamp: u64, user: Principal) -> Self {
        Self::new(
            id,
            params.event_type(),
            params.details(),
            timestamp,
            user,
        )
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

    fn save(&self) {
        save_event(Box::new(self.clone()));
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
            event_type,
            details,
            timestamp,
        }
    }

    pub fn from_params(id: u64, params: SystemEventParams, timestamp: u64) -> Self {
        Self::new(
            id,
            params.event_type(),
            params.details(),
            timestamp,
        )
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

    fn save(&self) {
        save_event(Box::new(self.clone()));
    }
}

impl ISystemEvent for SystemEvent {
    fn get_details(&self) -> SystemEventDetails {
        self.details.clone()
    }
}
