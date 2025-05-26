use async_trait::async_trait;
use types::CanisterId;
use candid::Nat;

use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};

#[async_trait]
pub trait LiquidityClient: Send + Sync {
    fn canister_id(&self) -> CanisterId;
    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, String>;
    async fn withdraw_liquidity_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, String>;
}
