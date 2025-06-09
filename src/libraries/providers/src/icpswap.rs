use types::CanisterId;
use candid::{Nat, Principal, Int};
use once_cell::sync::Lazy;

use icpswap_swap_factory_canister::{ICPSwapSwapFactoryResult, ICPSwapToken, ICPSwapPool};
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::ICPSwapSwapPoolResult;
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::claim::ClaimResponse;
use icpswap_swap_pool_canister::getUserUnusedBalance::UserUnusedBalance;
use icpswap_swap_pool_canister::getUserPositionsByPrincipal::UserPositionWithId;
use icpswap_swap_calculator_canister::getTokenAmountByLiquidity::GetTokenAmountByLiquidityResponse;
use icpswap_node_index_canister::getAllTokens::TokenData;
use icpswap_tvl_storage_canister::getPoolChartTvl::PoolChartTvl;
use utils::util::principal_to_canister_id;
use utils::constants::*;

// pub const SWAP_FACTORY_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 0, 208, 10, 215, 1, 1]);

pub static SWAP_FACTORY_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("4mmnk-kiaaa-aaaag-qbllq-cai"));
pub static SWAP_CALCULATOR_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("phr2m-oyaaa-aaaag-qjuoq-cai"));
pub static NODE_INDEX_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("ggzvv-5qaaa-aaaag-qck7a-cai"));

pub const SWAP_FEE: u128 = 3000;
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";

fn token_icpswap_format(token: &CanisterId) -> ICPSwapToken {
    let standard = if token.to_text() == ICP_TOKEN_CANISTER_ID {
        ICP_TOKEN_STANDARD.to_string()
    } else {
        ICRC2_TOKEN_STANDARD.to_string()
    };

    ICPSwapToken {
        address: token.to_text(),
        standard,
    }
}

// Swap Factory canister

pub async fn get_pool(token_in: CanisterId, token_out: CanisterId) -> Result<ICPSwapPool, String> {
    let pool_args = &icpswap_swap_factory_canister::getPool::Args {
        fee: candid::Nat::from(SWAP_FEE as u128),
        token0: token_icpswap_format(&token_in),
        token1: token_icpswap_format(&token_out),
    };

    match icpswap_swap_factory_canister_c2c_client::getPool(*SWAP_FACTORY_CANISTER, pool_args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapFactoryResult::Ok(pool) => {
                    Ok(pool)
                },
                ICPSwapSwapFactoryResult::Err(error) => {
                    Err(format!("ICPSwap get pool canister id failed 2: {:?}", error))
                },
            }
        },
        Err(e) => {
            Err(format!("ICPSwap get pool canister id failed 1: {:?}", e))
        },
    }
}

// Swap Pool canister

pub async fn quote(
    canister_id: CanisterId,
    amount_in: Nat,
    zero_for_one: bool,
    amount_out_minimum: Nat
) -> Result<Nat, String> {
        let quote_args = &icpswap_swap_pool_canister::quote::Args {
        amountIn: amount_in.to_string(),
        zeroForOne: zero_for_one,
        amountOutMinimum: amount_out_minimum.to_string(),
        };

        match icpswap_swap_pool_canister_c2c_client::quote(canister_id, quote_args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(amount_out_nat) => {
                        Ok(amount_out_nat)
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        Err(format!("Quote error 2 (ICPSWAP) : {:?} arguments {:?}", error, quote_args))
                    }
                }
            }
            Err(error) => {
                Err(format!("Quote error 1 (ICPSWAP) : {:?} arguments {:?}", error, quote_args))
            }
        }
    }

pub async fn swap(
    canister_id: CanisterId,
    amount_in: Nat,
    zero_for_one: bool,
    amount_out_minimum: Nat
) -> Result<Nat, String> {
    let swap_args = &icpswap_swap_pool_canister::swap::Args {
        amountIn: amount_in.to_string(),
        zeroForOne: zero_for_one,
        amountOutMinimum: amount_out_minimum.to_string(),
    };

    match icpswap_swap_pool_canister_c2c_client::swap(canister_id, swap_args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(amount_out_nat) => {
                    Ok(amount_out_nat)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Swap error 2 (ICPSWAP) : {:?} arguments {:?}", error, swap_args))
                }
            }
        }
        Err(error) => {
            Err(format!("Swap error 1 (ICPSWAP) : {:?} arguments {:?}", error, swap_args))
        }
    }
}

pub async fn get_token_meta(canister_id: CanisterId) -> Result<TokenMeta, String> {
    match icpswap_swap_pool_canister_c2c_client::getTokenMeta(canister_id).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(token_meta) => {
                    Ok(token_meta)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("ICPSwap get token meta canister id failed 2: {:?}", error))
                }
            }
        }
        Err(e) => {
            Err(format!("ICPSwap get token meta canister id failed 1: {:?}", e))
        }
    }
}

pub async fn deposit_from(
    canister_id: CanisterId,
    token_in: CanisterId,
    amount: Nat,
    token_fee: Nat
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::depositFrom::Args {
        token: token_in.to_text(),
        amount: amount,
        fee: token_fee,
    };

    match icpswap_swap_pool_canister_c2c_client::depositFrom(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(deposited_amount_nat) => {
                    Ok(deposited_amount_nat)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Deposit from error 2 (ICPSWAP) : {:?} arguments {:?}", error, args))
                }
            }
        }
        Err(error) => {
            Err(format!("Deposit from error 1 (ICPSWAP) : {:?} arguments {:?}", error, args))
        }
    }
}

pub async fn withdraw(
    canister_id: CanisterId,
    token_out: CanisterId,
    amount: Nat,
    token_fee: Nat
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::withdraw::Args {
        token: token_out.to_text(),
        amount: amount,
        fee: token_fee,
    };

    match icpswap_swap_pool_canister_c2c_client::withdraw(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(withdrawn_amount_nat) => {
                    Ok(withdrawn_amount_nat)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Withdraw error 2 (ICPSWAP) : {:?} arguments {:?}", error, args))
                }
            }
        }
        Err(error) => {
            Err(format!("Withdraw error 1 (ICPSWAP) : {:?} arguments {:?}", error, args))
        }
    }
}

pub async fn metadata(canister_id: CanisterId) -> Result<Metadata, String> {
    match icpswap_swap_pool_canister_c2c_client::metadata(canister_id).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(metadata) => {
                    Ok(metadata)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Metadata error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Metadata error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn mint(
    canister_id: CanisterId, 
    token0: String, 
    token1: String, 
    amount0_desired: String, 
    amount1_desired: String, 
    fee: Nat, 
    tick_lower: Int,
    tick_upper: Int
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::mint::Args {
        fee,
        tickUpper: tick_upper,
        token0,
        token1,
        amount0Desired: amount0_desired,
        amount1Desired: amount1_desired,
        tickLower: tick_lower,
    };

    match icpswap_swap_pool_canister_c2c_client::mint(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(minted_amount_nat) => {
                    Ok(minted_amount_nat)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Mint error 2 (ICPSWAP) : {:?} arguments {:?}", error, args))
                }
            }
        }
        Err(error) => {
            Err(format!("Mint error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_user_position_ids_by_principal(canister_id: CanisterId, principal: Principal) -> Result<Vec<Nat>, String> {
    match icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal(canister_id, (principal,)).await {
        Ok(response) => {
            match response {
                (ICPSwapSwapPoolResult::Ok(position_ids),) => {
                    Ok(position_ids)
                }
                (ICPSwapSwapPoolResult::Err(error),) => {
                    Err(format!("Get user position ids by principal error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Get user position ids by principal error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_user_positions_by_principal(canister_id: CanisterId, principal: Principal) -> Result<Vec<UserPositionWithId>, String> {
    match icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal(canister_id, (principal,)).await {
        Ok(response) => {
            match response {
                (ICPSwapSwapPoolResult::Ok(user_positions),) => {
                    Ok(user_positions)
                }
                (ICPSwapSwapPoolResult::Err(error),) => {
                    Err(format!("Get user positions by principal error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Get user positions by principal error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_user_unused_balance(canister_id: CanisterId, principal: String) -> Result<UserUnusedBalance, String> {
    let args = &icpswap_swap_pool_canister::getUserUnusedBalance::Args {
        principal: principal,
    };

    match icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(user_unused_balance) => {
                    Ok(user_unused_balance)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Get user unused balance error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Get user unused balance error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn increase_liquidity(
    canister_id: CanisterId,
    position_id: Nat,
    amount0_desired: String,
    amount1_desired: String
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::increaseLiquidity::Args {
        positionId: position_id,
        amount0Desired: amount0_desired,
        amount1Desired: amount1_desired,
    };
    
    match icpswap_swap_pool_canister_c2c_client::increaseLiquidity(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(amount_out_nat) => {
                    Ok(amount_out_nat)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Increase liquidity error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Increase liquidity error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn decrease_liquidity(
    canister_id: CanisterId,
    position_id: Nat,
    liquidity: String,
) -> Result<DecreaseLiquidityResponse, String> {
    let args = &icpswap_swap_pool_canister::decreaseLiquidity::Args {
        positionId: position_id,
        liquidity: liquidity,
    };

    match icpswap_swap_pool_canister_c2c_client::decreaseLiquidity(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(decrease_liquidity_response) => {
                    Ok(decrease_liquidity_response)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Decrease liquidity error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Decrease liquidity error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_user_position(canister_id: CanisterId, position_id: Nat) -> Result<UserPosition, String> {
    match icpswap_swap_pool_canister_c2c_client::getUserPosition(canister_id, (position_id,)).await {
        Ok(response) => {
            match response {
                (ICPSwapSwapPoolResult::Ok(user_position),) => {
                    Ok(user_position)
                }
                (ICPSwapSwapPoolResult::Err(error),) => {
                    Err(format!("Get user position error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Get user position error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn claim(
    canister_id: CanisterId,
    position_id: Nat,
) -> Result<ClaimResponse, String> {
    let args = &icpswap_swap_pool_canister::claim::Args {
        positionId: position_id,
    };

    match icpswap_swap_pool_canister_c2c_client::claim(canister_id, args).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(claim_response) => {
                    Ok(claim_response)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Claim error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Claim error 1 (ICPSWAP) : {:?}", error))
        }
    }
}

// Swap Calculator canister

pub async fn get_price(sqrt_price_x96: Nat, token_0_decimals: Nat, token_1_decimals: Nat) -> Result<f64, String> {
    match icpswap_swap_calculator_canister_c2c_client::getPrice(
        *SWAP_CALCULATOR_CANISTER,
        (sqrt_price_x96, token_0_decimals, token_1_decimals)
    ).await {
        Ok(response) => {
            Ok(response.0)
        }
        Err(error) => {
            Err(format!("Get price error (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_token_amount_by_liquidity(
    sqrt_price_x96: Nat,
    tick_lower: Int,
    tick_upper: Int,
    liquidity: Nat
) -> Result<GetTokenAmountByLiquidityResponse, String> {
    match icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity(
        *SWAP_CALCULATOR_CANISTER,
        (sqrt_price_x96, tick_lower, tick_upper, liquidity)
    ).await {
        Ok(response) => {
            Ok(response.0)
        }
        Err(error) => {
            Err(format!("Get token amount by liquidity error (ICPSWAP) : {:?}", error))
        }
    }
}

// Node Index canister

pub async fn get_all_tokens() -> Result<Vec<TokenData>, String> {
    match icpswap_node_index_canister_c2c_client::getAllTokens(*NODE_INDEX_CANISTER).await {
        Ok(response) => {
            Ok(response)
        }
        Err(error) => {
            Err(format!("Get all tokens error (ICPSWAP) : {:?}", error))
        }
    }
}

pub async fn get_tvl_storage_canister() -> Result<Vec<String>, String> {
    match icpswap_node_index_canister_c2c_client::tvlStorageCanister(*NODE_INDEX_CANISTER).await {
        Ok(response) => {
            Ok(response)
        }
        Err(error) => {
            Err(format!("Get tvl storage canister error (ICPSWAP) : {:?}", error))
        }
    }
}

// TVL Storage canister

pub async fn get_pool_chart_tvl(
    canister_id: CanisterId,
    pool_canister_id: String,
    offset: Nat,
    limit: Nat
) -> Result<Vec<PoolChartTvl>, String> {
    match icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl(
        canister_id,
        (pool_canister_id, offset, limit)
    ).await {
        Ok(response) => {
            Ok(response.0)
        }
        Err(error) => {
            Err(format!("Get pool chart tvl error (ICPSWAP) : {:?}", error))
        }
    }
}