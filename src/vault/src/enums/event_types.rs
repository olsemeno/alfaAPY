use candid::{CandidType, Deserialize, Nat, Principal};
use types::CanisterId;
use serde::Serialize;
use types::exchange_id::ExchangeId;

// === Event params ===

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventParams {
    AddLiquidity {
        amount: Nat,
        pool_id: String,
        token0: CanisterId,
    },
    RemoveLiquidity {
        amount: Nat,
        pool_id: String,
        token0: CanisterId,
    },
    Swap {
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Nat,
        amount_out: Nat,
    },
}

impl UserEventParams {
    pub fn event_type(&self) -> UserEventType {
        match self {
            UserEventParams::AddLiquidity { .. } => UserEventType::AddLiquidity,
            UserEventParams::RemoveLiquidity { .. } => UserEventType::RemoveLiquidity,
            UserEventParams::Swap { .. } => UserEventType::Swap,
        }
    }

    pub fn details(&self) -> UserEventDetails {
        match self {
            UserEventParams::AddLiquidity { amount, pool_id, token0 } => UserEventDetails::AddLiquidity {
                amount: amount.clone(),
                pool_id: pool_id.clone(),
                token0: token0.clone(),
            },
            UserEventParams::RemoveLiquidity { amount, pool_id, token0 } => UserEventDetails::RemoveLiquidity {
                amount: amount.clone(),
                pool_id: pool_id.clone(),
                token0: token0.clone(),
            },
            UserEventParams::Swap { token_in, token_out, amount_in, amount_out } => UserEventDetails::Swap {
                token_in: token_in.clone(),
                token_out: token_out.clone(),
                amount_in: amount_in.clone(),
                amount_out: amount_out.clone(),
            },
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventParams {
    Rebalance {
        previous_pool_id: String,
        new_pool_id: String,
    },
    Swap {
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Nat,
        amount_out: Nat,
    },
}

impl SystemEventParams {
    pub fn event_type(&self) -> SystemEventType {
        match self {
            SystemEventParams::Rebalance { .. } => SystemEventType::Rebalance,
            SystemEventParams::Swap { .. } => SystemEventType::Swap,
        }
    }

    pub fn details(&self) -> SystemEventDetails {
        match self {
            SystemEventParams::Rebalance { previous_pool_id, new_pool_id } => SystemEventDetails::Rebalance {
                previous_pool_id: previous_pool_id.clone(),
                new_pool_id: new_pool_id.clone(),
            },
            SystemEventParams::Swap { token_in, token_out, amount_in, amount_out } => SystemEventDetails::Swap {
                token_in: token_in.clone(),
                token_out: token_out.clone(),
                amount_in: amount_in.clone(),
                amount_out: amount_out.clone(),
            },
        }
    }
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ErrorEventParams {
    ExternalProvider {
        context: String,
        message: String,
        provider: ExchangeId,
    },
    BusinessLogic {
        context: String,
        message: String,
    },
    Validation {
        context: String,
        message: String,
        field: Option<String>,
        value: Option<String>,
    },
    Access {
        context: String,
        message: String,
    },
    Infrastructure {
        context: String,
        message: String,
        component: Option<String>,
    },
    Unknown {
        context: String,
        message: String,
    },
}

impl ErrorEventParams {
    pub fn event_type(&self) -> ErrorEventType {
        match self {
            ErrorEventParams::ExternalProvider { .. } => ErrorEventType::ExternalService,
            ErrorEventParams::BusinessLogic { .. } => ErrorEventType::BusinessLogic,
            ErrorEventParams::Validation { .. } => ErrorEventType::Validation,
            ErrorEventParams::Access { .. } => ErrorEventType::Access,
            ErrorEventParams::Infrastructure { .. } => ErrorEventType::Infrastructure,
            ErrorEventParams::Unknown { .. } => ErrorEventType::Unknown,
        }
    }

    pub fn details(&self) -> ErrorEventDetails {
        match self {
            ErrorEventParams::ExternalProvider { context, message, provider } => ErrorEventDetails::ExternalProvider {
                context: context.clone(),
                message: message.clone(),
                provider: provider.clone(),
            },
            ErrorEventParams::BusinessLogic { context, message } => ErrorEventDetails::BusinessLogic {
                context: context.clone(),
                message: message.clone(),
            },
            ErrorEventParams::Validation { context, message, field, value } => ErrorEventDetails::Validation {
                context: context.clone(),
                message: message.clone(),
                field: field.clone(),
                value: value.clone(),
            },
            ErrorEventParams::Access { context, message } => ErrorEventDetails::Access {
                context: context.clone(),
                message: message.clone(),
            },
            ErrorEventParams::Infrastructure { context, message, component } => ErrorEventDetails::Infrastructure {
                context: context.clone(),
                message: message.clone(),
                component: component.clone(),
            },
            ErrorEventParams::Unknown { context, message } => ErrorEventDetails::Unknown {
                context: context.clone(),
                message: message.clone(),
            },
        }
    }
}

// === Event types ===

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventType {
    AddLiquidity,      // User added liquidity to the pool
    RemoveLiquidity,   // User removed liquidity from the pool
    Swap,              // User swapped tokens in the pool
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventType {
    Rebalance,   // System rebalanced the pool
    Swap,        // System swapped tokens in the pool
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ErrorEventType {
    ExternalService,    // External service error
    BusinessLogic,      // Business logic error
    Validation,         // Validation error
    Access,             // Access/permission error
    Infrastructure,     // Infrastructure/network error
    Unknown,            // Unknown error
}

// === Event details ===

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventDetails {
    AddLiquidity {
        amount: Nat,
        pool_id: String,
        token0: CanisterId,
    },
    RemoveLiquidity {
        amount: Nat,
        pool_id: String,
        token0: CanisterId,
    },
    Swap {
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Nat,
        amount_out: Nat,
    },
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventDetails {
    Rebalance {
        previous_pool_id: String,
        new_pool_id: String,
    },
    Swap {
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Nat,
        amount_out: Nat,
    },
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ErrorEventDetails {
    ExternalProvider {
        context: String,
        message: String,
        provider: ExchangeId,
    },
    BusinessLogic {
        context: String,
        message: String,
    },
    Validation {
        context: String,
        message: String,
        field: Option<String>,
        value: Option<String>,
    },
    Access {
        context: String,
        message: String,
    },
    Infrastructure {
        context: String,
        message: String,
        component: Option<String>,
    },
    Unknown {
        context: String,
        message: String,
    },
}
