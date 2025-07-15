use types::exchange_id::ExchangeId;
use types::CanisterId;
use utils::constants::KONGSWAP_CANISTER_ID;
use providers::providers_factory::ProviderImpls;

use crate::clients::kongswap::KongSwapLiquidityClient;
use crate::clients::icpswap::ICPSwapLiquidityClient;
use crate::liquidity_client::LiquidityClient;

pub async fn get_liquidity_client(
    provider_impls: ProviderImpls,
    token0: CanisterId,
    token1: CanisterId,
    provider: ExchangeId,
) -> Box<dyn LiquidityClient + 'static> {
    match provider {
        ExchangeId::KongSwap => Box::new(
            KongSwapLiquidityClient::new(
                provider_impls.clone(),
                *KONGSWAP_CANISTER_ID,
                token0.clone(), 
                token1.clone()
            )
        ),
        ExchangeId::ICPSwap => Box::new(
            ICPSwapLiquidityClient::new(
                provider_impls,
                token0.clone(), 
                token1.clone()
            ).with_pool().await.unwrap() // TODO: handle error
        ),
        _ => panic!("Unsupported provider"),
    }
}
