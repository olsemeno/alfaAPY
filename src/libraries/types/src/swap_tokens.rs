use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::{CanisterId, Milliseconds};
use crate::exchange_id::ExchangeId;
use crate::exchanges::TokenInfo;
use crate::pin_number::PinNumberWrapper;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub swap_id: u128,
    pub input_token: TokenInfo,
    pub output_token: TokenInfo,
    pub input_amount: u128,
    pub exchange_args: ExchangeArgs,
    pub min_output_amount: u128,
    pub pin: Option<PinNumberWrapper>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ExchangeArgs {
    ICPSwap(ICPSwapArgs),
    Sonic(SonicArgs),
    KongSwap(KongSwapArgs),
}

impl ExchangeArgs {
    pub fn exchange_id(&self) -> ExchangeId {
        match self {
            ExchangeArgs::ICPSwap(_) => ExchangeId::ICPSwap,
            ExchangeArgs::Sonic(_) => ExchangeId::Sonic,
            ExchangeArgs::KongSwap(_) => ExchangeId::KongSwap,
        }
    }

    pub fn swap_canister_id(&self) -> CanisterId {
        match self {
            ExchangeArgs::ICPSwap(a) => a.swap_canister_id,
            ExchangeArgs::Sonic(a) => a.swap_canister_id,
            ExchangeArgs::KongSwap(a) => a.swap_canister_id,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ICPSwapArgs {
    pub swap_canister_id: CanisterId,
    pub zero_for_one: bool,
}

pub type SonicArgs = ICPSwapArgs;
pub type KongSwapArgs = ICPSwapArgs;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    SwapFailed,
    PinRequired,
    PinIncorrect(Milliseconds),
    TooManyFailedPinAttempts(Milliseconds),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub amount_out: u128,
}
