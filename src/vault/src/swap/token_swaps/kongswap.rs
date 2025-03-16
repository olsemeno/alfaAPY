use std::fmt::format;
use super::swap_client::{SwapClient, SwapSuccess};
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat};
use ic_cdk::{call, trap};
use ic_cdk::api::call::CallResult;
use ic_response_codes::RejectCode;
use serde::Serialize;
use kongswap_canister::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::swap::SwapReply;
use types::exchanges::TokenInfo;
use types::CanisterId;
use crate::swap::swap_service::{KONG_BE_CANISTER, SNS_GOVERNANCE_CANISTER_ID};
use crate::swap::token_swaps::nat_to_u128;

pub struct KongSwapClient {
    canister_id: CanisterId,
    token_in: TokenInfo,
    token_out: TokenInfo,
}

impl KongSwapClient {
    pub fn new(canister_id: CanisterId, token_in: TokenInfo, token_out: TokenInfo) -> KongSwapClient {
        KongSwapClient {
            canister_id,
            token_in,
            token_out,
        }
    }
}

// #[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
// pub struct SwapArgs {
//     pub pay_token: String,
//     pub pay_amount: Nat,
//     // pub pay_tx_id: Option<TxId>,
//     pub receive_token: String,
//     pub receive_amount: Option<Nat>,
//     pub receive_address: Option<String>,
//     pub max_slippage: Option<f64>,
//     pub referred_by: Option<String>,
// }


#[async_trait]
impl SwapClient for KongSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: u128) -> Result<Result<SwapSuccess, String>, (RejectCode, String)> {


let args = &kongswap_canister::swap::Args {
    pay_amount: amount.into(),
    pay_token: format!("IC.{}", self.token_in.ledger),
    receive_token: format!("IC.{}", self.token_out.ledger),
    max_slippage: Some(10.0),
};


        match kongswap_canister_c2c_client::swap(
            self.canister_id,
            args,
        )
        .await
        {
            Ok(response) => {
                match response {
                    Ok(response) => {
                        let amount_out = nat_to_u128(response.receive_amount);
                        Ok(Ok(SwapSuccess {
                            amount_out,
                            withdrawal_success: Some(response.claim_ids.is_empty()),
                        }))
                    }
                    Err(error) => {
                        trap(format!("Swap error 3 : {:?} arguments {:?}", error, args).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("Swap error 4 : {:?} arguments {:?}", error, args).as_str());
            },
        }
    }
}
