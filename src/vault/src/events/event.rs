use candid::Principal;
use candid::{CandidType, Deserialize};
use serde::Serialize;
use crate::enums::{UserEventType, UserEventDetails, SystemEventType, SystemEventDetails};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserEvent {
    pub id: u64,
    pub event_type: UserEventType,
    pub timestamp: u64,
    pub details: UserEventDetails,
    pub user: Principal,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SystemEvent {
    pub id: u64,
    pub event_type: SystemEventType,
    pub details: SystemEventDetails,
    pub timestamp: u64,
}

pub trait IUserEvent {
    fn get_id(&self) -> u64;
    fn get_timestamp(&self) -> u64;
    fn get_user(&self) -> Principal;
    fn get_details(&self) -> UserEventDetails;
}

pub trait ISystemEvent {
    fn get_id(&self) -> u64;
    fn get_timestamp(&self) -> u64;
    fn get_details(&self) -> SystemEventDetails;
}

impl IUserEvent for UserEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_user(&self) -> Principal {
        self.user
    }

    fn get_details(&self) -> UserEventDetails {
        self.details.clone()
    }
}

impl ISystemEvent for SystemEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_details(&self) -> SystemEventDetails {
        self.details.clone()
    }
}
