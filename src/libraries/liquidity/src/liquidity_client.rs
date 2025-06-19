use async_trait::async_trait;
use types::CanisterId;
use candid::Nat;

use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse, GetPositionByIdResponse, GetPoolDataResponse};
use errors::internal_error::error::InternalError;

#[async_trait]
pub trait LiquidityClient: Send + Sync {
    fn canister_id(&self) -> CanisterId;
    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, InternalError>;
    async fn withdraw_liquidity_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, InternalError>;
    async fn get_position_by_id(&self, position_id: u64) -> Result<GetPositionByIdResponse, InternalError>;
    async fn get_pool_data(&self) -> Result<GetPoolDataResponse, InternalError>;
}
