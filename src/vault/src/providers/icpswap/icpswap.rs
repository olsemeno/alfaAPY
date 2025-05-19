use types::CanisterId;
use candid::Nat;

use crate::swap::token_swaps::nat_to_u128;
use types::exchanges::TokenInfo;
use icpswap_swap_factory_canister::{ICPSwapSwapFactoryResult, ICPSwapToken, ICPSwapPool};
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::ICPSwapSwapPoolResult;
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::claim::ClaimResponse;

pub const SWAP_FACTORY_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 0, 208, 10, 215, 1, 1]);
pub const SWAP_CALCULATOR_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 0, 208, 10, 215, 1, 1]); // TODO: change to real canister id
pub const SWAP_FEE: u128 = 3000;
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";

fn make_icpswap_token(token: &TokenInfo) -> ICPSwapToken {
    let standard = match token.symbol.as_str() {
        "ICP" => ICP_TOKEN_STANDARD.to_string(),
        _ => ICRC2_TOKEN_STANDARD.to_string(),
    };

    ICPSwapToken {
        address: token.ledger.to_string(),
        standard,
    }
}

// Swap Factory canister

pub async fn get_pool(token_in: TokenInfo, token_out: TokenInfo) -> Result<ICPSwapPool, String> {
    let pool_args = &icpswap_swap_factory_canister::getPool::Args {
        fee: candid::Nat::from(SWAP_FEE as u128),
        token0: make_icpswap_token(&token_in),
        token1: make_icpswap_token(&token_out),
    };

    match icpswap_swap_factory_canister_c2c_client::getPool(SWAP_FACTORY_CANISTER, pool_args).await {
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
    token_in: TokenInfo,
    amount: Nat,
    token_fee: Nat
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::depositFrom::Args {
        token: token_in.ledger.to_string(),
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
    token_out: TokenInfo,
    amount: Nat,
    token_fee: Nat
) -> Result<Nat, String> {
    let args = &icpswap_swap_pool_canister::withdraw::Args {
        token: token_out.ledger.to_string(),
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
    tick_lower: i32, 
    tick_upper: i32
) -> Result<u128, String> {
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
                    Ok(nat_to_u128(minted_amount_nat))
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

pub async fn get_user_position_ids_by_principal(canister_id: CanisterId, principal: String) -> Result<Vec<Nat>, String> {
    match icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal(canister_id, &(principal,)).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(position_ids) => {
                    Ok(position_ids)
                }
                ICPSwapSwapPoolResult::Err(error) => {
                    Err(format!("Get user position ids by principal error 2 (ICPSWAP) : {:?}", error))
                }
            }
        }
        Err(error) => {
            Err(format!("Get user position ids by principal error 1 (ICPSWAP) : {:?}", error))
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
    match icpswap_swap_pool_canister_c2c_client::getUserPosition(canister_id, &(position_id,)).await {
        Ok(response) => {
            match response {
                ICPSwapSwapPoolResult::Ok(user_position) => {
                    Ok(user_position)
                }
                ICPSwapSwapPoolResult::Err(error) => {
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
    match icpswap_swap_calculator_canister_c2c_client::getPrice(SWAP_CALCULATOR_CANISTER, &(sqrt_price_x96, token_0_decimals, token_1_decimals)).await {
        Ok(response) => {
            Ok(response.0)
        }
        Err(error) => {
            Err(format!("Get price error (ICPSWAP) : {:?}", error))
        }
    }
}