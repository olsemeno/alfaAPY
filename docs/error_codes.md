# Error Codes Documentation

The project uses a structured error code format:

```
MMMMKKNN
```

Where:
- **MMMM** — Module (see table below)
- **KK**   — Error Kind (2 digits)
- **NN**   — Error Number (2 digits, unique within module+kind)

## Code Blocks

### Module (MMMM)

| Code | Module                         |
|------|--------------------------------|
| 1001 | External Service (KongSwap)    |
| 1002 | External Service (ICPSwap)     |
| 1100 | External Service (ICRC Ledger) |
| 1200 | External Service (Canister)    |
| 2000 | Swap (Service)                 |
| 2001 | Swap (KongSwap)                |
| 2002 | Swap (ICPSwap)                 |
| 2101 | Liquidity (KongSwap)           |
| 2102 | Liquidity (ICPSwap)            |
| 2200 | Validation                     |
| 2301 | Mock Provider (KongSwap)       |
| 2302 | Mock Provider (ICPSwap)       |
| 3000 | Service (Vault)                |
| 3100 | Strategies (Vault)             |
| 4000 | Service (PoolStats)            |
| 4100 | PoolMetrics (PoolStats)        |

### Error Kind (KK)
| Code | Kind            |
|------|-----------------|
| 01   | NotFound        |
| 02   | Validation      |
| 03   | BusinessLogic   |
| 04   | ExternalService |
| 05   | AccessDenied    |
| 06   | Infrastructure  |
| 07   | Timeout         |

### Error Number (NN)
- Unique error number within the module (01, 02, ...)

---

## Example Error Code

```
21020301
```
- **2102** — Liquidity (ICPSwap)
- **03**   — BusinessLogic
- **01**   — First error for Liquidity (ICPSwap) / BusinessLogic

---

## Usage Recommendations
- Store the error code as `u32` (or `nat32` in Candid).
- Document the meaning of all code blocks for each error code.
- Do not use a string for storing the code — only for displaying it.
- Use a builder function for error codes: `build_error_code(module, kind, number)`

---

## Examples

| Code     | Module                | Kind            | Number | Description                        |
|----------|-----------------------|-----------------|--------|------------------------------------|
| 10010401 | External (KongSwap)   | ExternalService | 01     | KongSwap external call failed      |
| 21020301 | Liquidity (ICPSwap)   | BusinessLogic   | 01     | Invalid token configuration        |
| 30000301 | Strategies (Vault)    | BusinessLogic   | 01     | Strategy logic error               |
| 40000101 | Liquidity (PoolStats) | NotFound        | 01     | PoolStats resource not found       |

---

## Extension
- When adding new modules, error kinds, or error numbers — update this documentation.

---

# Error code list

## 1001 - External Service (KongSwap)

**1001 04 01** - IC error calling kongswap_canister_c2c_client::pools from KongSwapProvider::pools (External Service)  
**1001 03 02** - Error calling kongswap_canister_c2c_client::pools from KongSwapProvider::pools (Business Logic)  

**1001 04 03** - IC error calling kongswap_canister_c2c_client::swap_amounts from KongSwapProvider::swap_amounts (External Service)  
**1001 03 04** - Error calling kongswap_canister_c2c_client::swap_amounts from KongSwapProvider::swap_amounts (Business Logic)  

**1001 04 05** - IC error calling kongswap_canister_c2c_client::swap from KongSwapProvider::swap (External Service)  
**1001 03 06** - Error calling kongswap_canister_c2c_client::swap from KongSwapProvider::swap (Business Logic)  

**1001 04 07** - IC error calling kongswap_canister_c2c_client::add_liquidity_amounts from KongSwapProvider::add_liquidity_amounts (External Service)  
**1001 03 08** - Error calling kongswap_canister_c2c_client::add_liquidity_amounts from KongSwapProvider::add_liquidity_amounts (Business Logic)  

**1001 04 09** - IC error calling kongswap_canister_c2c_client::add_liquidity from KongSwapProvider::add_liquidity (External Service)  
**1001 03 10** - Error calling kongswap_canister_c2c_client::add_liquidity from KongSwapProvider::add_liquidity (Business Logic)  

**1001 04 11** - IC error calling kongswap_canister_c2c_client::user_balances from KongSwapProvider::user_balances (External Service)  
**1001 03 12** - Error calling kongswap_canister_c2c_client::user_balances from KongSwapProvider::user_balances (Business Logic)  

**1001 04 13** - IC error calling kongswap_canister_c2c_client::remove_liquidity_amounts from KongSwapProvider::remove_liquidity_amounts (External Service)  
**1001 03 14** - Error calling kongswap_canister_c2c_client::remove_liquidity_amounts from KongSwapProvider::remove_liquidity_amounts (Business Logic) 

**1001 04 15** - IC error calling kongswap_canister_c2c_client::remove_liquidity from KongSwapProvider::remove_liquidity (External Service)  
**1001 03 16** - Error calling kongswap_canister_c2c_client::remove_liquidity from KongSwapProvider::remove_liquidity (Business Logic)  

## 1002 - External Service (ICPSwap)

**1002 04 01** - IC error calling icpswap_swap_factory_canister_c2c_client::getPool from ICPSwapProvider::get_pool (External Service)  
**1002 03 02** - Error calling icpswap_swap_factory_canister_c2c_client::getPool from ICPSwapProvider::get_pool (Business Logic)  

**1002 04 03** - IC error calling icpswap_swap_pool_canister_c2c_client::quote from ICPSwapProvider::quote (External Service)  
**1002 03 04** - Error calling icpswap_swap_pool_canister_c2c_client::quote from ICPSwapProvider::quote (Business Logic)  

**1002 04 05** - IC error calling icpswap_swap_pool_canister_c2c_client::swap from ICPSwapProvider::swap (External Service)  
**1002 03 06** - Error calling icpswap_swap_pool_canister_c2c_client::swap from ICPSwapProvider::swap (Business Logic)  

**1002 04 07** - IC error calling icpswap_swap_pool_canister_c2c_client::getTokenMeta from ICPSwapProvider::get_token_meta (External Service)  
**1002 03 08** - Error calling icpswap_swap_pool_canister_c2c_client::getTokenMeta from ICPSwapProvider::get_token_meta (Business Logic)  

**1002 04 09** - IC error calling icpswap_swap_pool_canister_c2c_client::depositFrom from ICPSwapProvider::deposit_from (External Service)  
**1002 03 10** - Error calling icpswap_swap_pool_canister_c2c_client::depositFrom from ICPSwapProvider::deposit_from (Business Logic) 

**1002 04 11** - IC error calling icpswap_swap_pool_canister_c2c_client::withdraw from ICPSwapProvider::withdraw (External Service)  
**1002 03 12** - Error calling icpswap_swap_pool_canister_c2c_client::withdraw from ICPSwapProvider::withdraw (Business Logic)  

**1002 04 13** - IC error calling icpswap_swap_pool_canister_c2c_client::metadata from ICPSwapProvider::metadata (External Service)  
**1002 03 14** - Error calling icpswap_swap_pool_canister_c2c_client::metadata from ICPSwapProvider::metadata (Business Logic)  

**1002 04 15** - IC error calling icpswap_swap_pool_canister_c2c_client::mint from ICPSwapProvider::mint (External Service)  
**1002 03 16** - Error calling icpswap_swap_pool_canister_c2c_client::mint from ICPSwapProvider::mint (Business Logic)  

**1002 04 17** - IC error calling icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal from ICPSwapProvider::get_user_position_ids_by_principal (External Service)  
**1002 03 18** - Error calling icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal from ICPSwapProvider::get_user_position_ids_by_principal (Business Logic)  

**1002 04 19** - IC error calling icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal from ICPSwapProvider::get_user_positions_by_principal (External Service)  
**1002 03 20** - Error calling icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal from ICPSwapProvider::get_user_positions_by_principal (Business Logic)  

**1002 04 21** - IC error calling icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance from ICPSwapProvider::get_user_unused_balance (External Service)  
**1002 03 22** - Error calling icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance from ICPSwapProvider::get_user_unused_balance (Business Logic)  

**1002 04 23** - IC error calling icpswap_swap_pool_canister_c2c_client::increaseLiquidity from ICPSwapProvider::increase_liquidity (External Service)  
**1002 03 24** - Error calling icpswap_swap_pool_canister_c2c_client::increaseLiquidity from ICPSwapProvider::increase_liquidity (Business Logic)  

**1002 04 25** - IC error calling icpswap_swap_pool_canister_c2c_client::decreaseLiquidity from ICPSwapProvider::decrease_liquidity (External Service)  
**1002 03 26** - Error calling icpswap_swap_pool_canister_c2c_client::decreaseLiquidity from ICPSwapProvider::decrease_liquidity (Business Logic)  

**1002 04 27** - IC error calling icpswap_swap_pool_canister_c2c_client::getUserPosition from ICPSwapProvider::get_user_position (External Service)  
**1002 03 28** - Error calling icpswap_swap_pool_canister_c2c_client::getUserPosition from ICPSwapProvider::get_user_position (Business Logic)  

**1002 04 29** - IC error calling icpswap_swap_pool_canister_c2c_client::claim from ICPSwapProvider::claim (External Service)  
**1002 03 30** - Error calling icpswap_swap_pool_canister_c2c_client::claim from ICPSwapProvider::claim (Business Logic)  

**1002 04 31** - IC error calling icpswap_swap_calculator_canister_c2c_client::getPrice from ICPSwapProvider::get_price (External Service)  
**1002 03 32** - Error calling icpswap_swap_calculator_canister_c2c_client::getPrice from ICPSwapProvider::get_price (Business Logic)  

**1002 04 33** - IC error calling icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity from ICPSwapProvider::get_token_amount_by_liquidity (External Service)  
**1002 03 34** - Error calling icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity from ICPSwapProvider::get_token_amount_by_liquidity (Business Logic)  

**1002 04 35** - IC error calling icpswap_node_index_canister_c2c_client::getAllTokens from ICPSwapProvider::get_all_tokens (External Service)  
**1002 03 36** - Error calling icpswap_node_index_canister_c2c_client::getAllTokens from ICPSwapProvider::get_all_tokens (Business Logic)  

**1002 04 37** - IC error calling icpswap_global_index_canister_c2c_client::tvlStorageCanister from ICPSwapProvider::get_tvl_storage_canister (External Service)
**1002 03 38** - Error calling icpswap_global_index_canister_c2c_client::tvlStorageCanister from ICPSwapProvider::get_tvl_storage_canister (Business Logic)

**1002 04 39** - IC error calling icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl from ICPSwapProvider::get_pool_chart_tvl (External Service)  
**1002 03 40** - Error calling icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl from ICPSwapProvider::get_pool_chart_tvl (Business Logic)  


## 1100 - External Service (ICRC Ledger)

**1100 04 01** - IC error calling icrc_ledger_canister_c2c_client::icrc1_decimals from icrc_ledger_client::icrc1_decimals (External Service)

**1100 04 02** - IC error calling icrc_ledger_canister_c2c_client::icrc2_approve from icrc_ledger_client::icrc2_approve (External Service)
**1100 03 03** - Error calling icrc_ledger_canister_c2c_client::icrc2_approve from icrc_ledger_client::icrc2_approve (Business Logic)

**1100 04 04** - IC error calling icrc_ledger_canister_c2c_client::icrc2_transfer_from from icrc_ledger_client::icrc2_transfer_from (External Service)
**1100 03 05** - Error calling icrc_ledger_canister_c2c_client::icrc2_transfer_from from icrc_ledger_client::icrc2_transfer_from (Business Logic)

**1100 04 06** - IC error calling icrc_ledger_canister_c2c_client::icrc1_fee from icrc_ledger_client::icrc1_fee (External Service)


## 1200 - External Service (Canister)

**1200 04 01** - IC error calling canister_client::make_c2c_call from Utils::icrc1_transfer_to_user (External Service)  
**1200 03 02** - Error calling canister_client::make_c2c_call from Utils::icrc1_transfer_to_user (Business Logic)  


## 2000 - Swap (Service)

**2000 03 01** - Invalid provider in swap_service::swap_icrc2 (BusinessLogic)

**2000 03 02** - Invalid provider in swap_service::quote_swap_icrc2 (BusinessLogic)


## 2001 - Swap (KongSwap)



## 2002 - Swap (ICPSwap)

**2002 03 01** - Invalid token configuration for ICPSwap pool in ICPSwapSwapClient::is_zero_for_one_swap_direction (Business Logic) 

**2002 03 02** - Invalid token configuration for ICPSwap pool in ICPSwapSwapClient::get_tokens_fee (Business Logic)  


## 2101 - Liquidity (KongSwap)

**2101 03 01** - No user LP balance in KongSwapLiquidityClient::withdraw_liquidity_from_pool (Business Logic)  

**2101 03 02** - No user LP balance in KongSwapLiquidityClient::get_position_by_id (Business Logic)  

**2101 03 03** - No pool data in KongSwapLiquidityClient::get_pool_data (Business Logic)  


## 2102 - Liquidity (ICPSwap)

**2102 03 01** - Invalid token configuration for ICPSwap pool in ICPSwapLiquidityClient::get_tokens_fee (Business Logic)  

**2102 03 02** - Invalid token configuration for ICPSwap pool in ICPSwapLiquidityClient::is_zero_for_one_swap_direction (Business Logic)  

**2102 03 03** - Token order does not match pool metadata in ICPSwapLiquidityClient::add_liquidity_to_pool (Business Logic)  

**2102 03 04** - No position ids found for user in ICPSwapLiquidityClient::withdraw_liquidity_from_pool (Business Logic)  

**2102 03 05** - Token order does not match pool metadata in ICPSwapLiquidityClient::withdraw_liquidity_from_pool (Business Logic)  


## 2200 - Validation

**2200 02 01** - Field validation failed in FieldValidator::validate (Validation)


## 2301 - Mock Provider (KongSwap)

**2301 01 01** - Mock response not set for swap_amounts in MockKongSwapProvider::swap_amounts (NotFound) 

**2301 01 02** - Mock response not set for swap in MockKongSwapProvider::swap (NotFound)  

**2301 01 03** - Mock response not set for add_liquidity_amounts in MockKongSwapProvider::add_liquidity_amounts (NotFound)  

**2301 01 04** - Mock response not set for add_liquidity in MockKongSwapProvider::add_liquidity (NotFound)  

**2301 01 05** - Mock response not set for user_balances in MockKongSwapProvider::user_balances (NotFound)  

**2301 01 06** - Mock response not set for remove_liquidity_amounts in MockKongSwapProvider::remove_liquidity_amounts (NotFound)  

**2301 01 07** - Mock response not set for remove_liquidity in MockKongSwapProvider::remove_liquidity (NotFound)  


## 2302 - Mock Provider (ICPSwap)

**2302 01 01** - Mock response not set for get_all_tokens in MockICPSwapProvider::get_all_tokens (NotFound)  

**2302 01 02** - Mock response not set for get_tvl_storage_canister in MockICPSwapProvider::get_tvl_storage_canister (NotFound)  

**2302 01 03** - Mock response not set for get_pool in MockICPSwapProvider::get_pool (NotFound)  

**2302 01 04** - Mock response not set for quote in MockICPSwapProvider::quote (NotFound)  

**2302 01 05** - Mock response not set for swap in MockICPSwapProvider::swap (NotFound)  

**2302 01 06** - Mock response not set for get_token_meta in MockICPSwapProvider::get_token_meta (NotFound)  

**2302 01 07** - Mock response not set for deposit_from in MockICPSwapProvider::deposit_from (NotFound)  

**2302 01 08** - Mock response not set for withdraw in MockICPSwapProvider::withdraw (NotFound)  

**2302 01 09** - Mock response not set for metadata in MockICPSwapProvider::metadata (NotFound)  

**2302 01 10** - Mock response not set for mint in MockICPSwapProvider::mint (NotFound)  

**2302 01 11** - Mock response not set for get_user_position_ids_by_principal in MockICPSwapProvider::get_user_position_ids_by_principal (NotFound)  

**2302 01 12** - Mock response not set for get_user_positions_by_principal in MockICPSwapProvider::get_user_positions_by_principal (NotFound)  

**2302 01 13** - Mock response not set for get_user_unused_balance in MockICPSwapProvider::get_user_unused_balance (NotFound)  

**2302 01 14** - Mock response not set for increase_liquidity in MockICPSwapProvider::increase_liquidity (NotFound)  

**2302 01 15** - Mock response not set for decrease_liquidity in MockICPSwapProvider::decrease_liquidity (NotFound)  

**2302 01 16** - Mock response not set for get_user_position in MockICPSwapProvider::get_user_position (NotFound)  

**2302 01 17** - Mock response not set for claim in MockICPSwapProvider::claim (NotFound)  

**2302 01 18** - Mock response not set for get_price in MockICPSwapProvider::get_price (NotFound)  

**2302 01 19** - Mock response not set for get_token_amount_by_liquidity in MockICPSwapProvider::get_token_amount_by_liquidity (NotFound)  

**2302 01 20** - Mock response not set for get_pool_chart_tvl in MockICPSwapProvider::get_pool_chart_tvl (NotFound)

## 3000 - Service (Vault)

**3000 01 01** - Strategy not found in service::deposit (NotFound) 

**3000 01 02** - Strategy not found in service::withdraw (NotFound)  


## 3100 - Strategies (Vault)

**3100 01 01** - No pool found to deposit in Strategy::deposit (NotFound)  
**3100 01 02** - **DEPRECATED** No current pool found to deposit in Strategy::deposit (NotFound)  

**3100 03 03** - No shares found for user in Strategy::withdraw (BusinessLogic)  
**3100 03 04** - Not sufficient shares for user in Strategy::withdraw (BusinessLogic)  
**3100 01 05** - No current pool found in strategy in Strategy::withdraw (NotFound)  

**3100 01 06** - No current pool found in strategy in Strategy::rebalance (NotFound)  

**3100 03 07** - Strategy has no current pool in strategy_stats_service::get_strategy_current_liquidity (BusinessLogic)
**3100 03 08** - Strategy has no position id in strategy_stats_service::get_strategy_current_liquidity (BusinessLogic)

## 4000 - Service (PoolStats)

**4000 01 01** - Pool not found in service::delete_pool (NotFound)  

**4000 01 02** - Pool not found in service::get_pool_by_id (NotFound)  

**4000 01 03** - Pool not found in service::add_liquidity_to_pool (NotFound) 
**4000 03 04** - Pool already has liquidity in service::add_liquidity_to_pool (BusinessLogic) 

**4000 01 05** - Pool not found in service::withdraw_liquidity_from_pool (NotFound)  
**4000 03 06** - Pool has no liquidity in service::withdraw_liquidity_from_pool (BusinessLogic) 

## 4100 - PoolMetrics (PoolStats)

**4100 03 01** – Pool has no position_id in pool_snapshot_service::create_pool_snapshot (BusinessLogic)
