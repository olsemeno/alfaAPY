use crate::liquidity::clients::kongswap::KongSwapClient;
use crate::liquidity::liquidity_client::LiquidityClient;
use kongswap_canister::PoolReply;
use types::CanisterId;
use std::sync::Arc;

pub enum Provider {
    KongSwap,
    ICPswap,
}

pub struct LiquidityRouter {
    kong_canister_id: CanisterId,
    // icpswap_canister_id: CanisterId,
    // ...
}

impl LiquidityRouter {
    pub fn new(pool: PoolReply) -> Self {
        Self {
            kong_canister_id,
            // icpswap_canister_id,
        }
    }

    pub fn get_client(
        &self,
        pool: PoolReply,
    ) -> Arc<dyn LiquidityClient + Send + Sync> {
        match provider {
            LiquidityProvider::KongSwap => {
                Arc::new(KongSwapClient::new(self.kong_canister_id, pool))
            }
            // LiquidityProvider::ICPSwap => Arc::new(ICPSwapLiquidityClient::new(...)),
        }
    }
}