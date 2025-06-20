use crate::clients::kongswap::KongSwapLiquidityClient;
use crate::clients::icpswap::ICPSwapLiquidityClient;
use crate::liquidity_client::LiquidityClient;
use types::exchange_id::ExchangeId;
use types::CanisterId;
use utils::constants::KONGSWAP_CANISTER_ID;

pub async fn get_liquidity_client(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Box<dyn LiquidityClient> {
    match provider {
        ExchangeId::KongSwap => Box::new(
            KongSwapLiquidityClient::new(
                *KONGSWAP_CANISTER_ID,
                token0.clone(), 
                token1.clone()
            )
        ),
        ExchangeId::ICPSwap => Box::new(
            ICPSwapLiquidityClient::new(
                token0.clone(), 
                token1.clone()
            ).with_pool().await.unwrap() // TODO: handle error
        ),
        _ => panic!("Unsupported provider"),
    }
}
