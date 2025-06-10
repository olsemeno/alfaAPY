use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ic_cdk::api::time;


use crate::events::event_candid::EventCandid;
use crate::types::types::EventResponse;
use crate::repository::events_repo;
use crate::enums::{
    UserEventType,
    UserEventDetails,
    SystemEventType,
    SystemEventDetails,
    UserEventParams,
    SystemEventParams,
    ErrorEventType,
    ErrorEventDetails,
    ErrorEventParams,
};


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

pub trait IErrorEvent: IEvent {
    fn get_user(&self) -> Option<Principal>;
    fn get_details(&self) -> ErrorEventDetails;
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

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ErrorEvent {
    pub id: u64,
    pub timestamp: u64,
    pub event_type: ErrorEventType,
    pub details: ErrorEventDetails,
    pub user: Option<Principal>,
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

    pub fn build(id: u64, params: UserEventParams, timestamp: u64, user: Principal) -> Self {
        Self::new(
            id,
            params.event_type(),
            params.details(),
            timestamp,
            user,
        )
    }

    pub fn create(params: UserEventParams, user: Principal) -> Self {
        let event = Self::build(
            events_repo::get_user_events_count(),
            params,
            time(),
            user,
        );

        event.save();
        event
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
        events_repo::save_event(Box::new(self.clone()));
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

    pub fn build(id: u64, params: SystemEventParams, timestamp: u64) -> Self {
        Self::new(
            id,
            params.event_type(),
            params.details(),
            timestamp,
        )
    }

    pub fn create(params: SystemEventParams) -> Self {
        let event = Self::build(
            events_repo::get_system_events_count(),
            params,
            time(),
        );

        event.save();
        event
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
        events_repo::save_event(Box::new(self.clone()));
    }
}

impl ISystemEvent for SystemEvent {
    fn get_details(&self) -> SystemEventDetails {
        self.details.clone()
    }
}

// ErrorEvent implementation

impl ErrorEvent {
    pub fn new(id: u64, event_type: ErrorEventType, details: ErrorEventDetails, timestamp: u64, user: Option<Principal>) -> Self {
        Self {
            id,
            event_type,
            details,
            timestamp,
            user,
        }
    }

    pub fn build(id: u64, params: ErrorEventParams, timestamp: u64, user: Option<Principal>) -> Self {
        Self::new(id, params.event_type(), params.details(), timestamp, user)
    }

    pub fn create(params: ErrorEventParams, user: Option<Principal>) -> Self {
        let event = Self::build(
            events_repo::get_error_events_count(),
            params,
            time(),
            user,
        );
        event.save();
        event
    }
}

impl IEvent for ErrorEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn to_candid(&self) -> EventCandid {
        EventCandid::Error(self.clone())
    }

    fn to_response(&self) -> EventResponse {
        EventResponse::Error(self.clone())
    }

    fn save(&self) {
        events_repo::save_event(Box::new(self.clone()));
    }
}

impl IErrorEvent for ErrorEvent {
    fn get_user(&self) -> Option<Principal> {
        self.user
    }

    fn get_details(&self) -> ErrorEventDetails {
        self.details.clone()
    }
}
