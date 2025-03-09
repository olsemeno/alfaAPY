use ic_cdk::trap;
use kongswap_canister::PoolReply;
use crate::providers::kong::kong::pools;
use crate::strategies::strategy::Pool;

pub async fn get_pools_data(required_pools: Vec<Pool>) -> Vec<PoolReply> {
    match pools().await {
        Ok(response) => {
            let pools = response.pools;
            let mut pool_data = Vec::new();
            for pool in required_pools {
                match  pools.iter().find(|&x| x.symbol == pool.pool_symbol)
                {
                    None => {}
                    Some(x) => {
                        pool_data.push(x.to_owned());
                    }
                }
            }
            pool_data
        }
        Err(error) => {
            trap(error.as_str());
        }
    }
}