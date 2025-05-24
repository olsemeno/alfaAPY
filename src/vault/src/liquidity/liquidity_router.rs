use crate::liquidity::clients::kongswap::KongSwapLiquidityClient;
use crate::liquidity::clients::icpswap::ICPSwapLiquidityClient;
use crate::liquidity::liquidity_client::LiquidityClient;
use crate::pools::pool::Pool;
use types::exchange_id::ExchangeId;
use crate::swap::swap_service::KONG_BE_CANISTER;

pub async fn get_liquidity_client(pool: &Pool) -> Box<dyn LiquidityClient> {
    match pool.provider {
        ExchangeId::KongSwap => Box::new(
            KongSwapLiquidityClient::new(
                KONG_BE_CANISTER, 
                pool.token0.clone(), 
                pool.token1.clone()
            )
        ),
        ExchangeId::ICPSwap => Box::new(
            ICPSwapLiquidityClient::new(
                pool.token0.clone(), 
                pool.token1.clone()
            ).await
        ),
        _ => panic!("Unsupported provider"),
    }
}
