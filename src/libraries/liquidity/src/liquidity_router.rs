use crate::clients::kongswap::KongSwapLiquidityClient;
use crate::clients::icpswap::ICPSwapLiquidityClient;
use crate::liquidity_client::LiquidityClient;
use types::exchange_id::ExchangeId;
use types::exchanges::TokenInfo;
use providers::kongswap::KONGSWAP_CANISTER;

pub async fn get_liquidity_client(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Box<dyn LiquidityClient> {
    match provider {
        ExchangeId::KongSwap => Box::new(
            KongSwapLiquidityClient::new(
                *KONGSWAP_CANISTER,
                token0.clone(), 
                token1.clone()
            )
        ),
        ExchangeId::ICPSwap => Box::new(
            ICPSwapLiquidityClient::new(
                token0.clone(), 
                token1.clone()
            ).await
        ),
        _ => panic!("Unsupported provider"),
    }
}
