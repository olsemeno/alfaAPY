# Calculator Functions Documentation

## Overview

The Calculator module provides essential mathematical functions for the AlfaAPY protocol, handling share calculations and liquidity distribution across pools. These calculations are critical for ensuring accurate user share allocation and optimal liquidity distribution.

## Functions

### 1. `calculate_shares`

This function calculates the number of shares a user receives for their deposit based on the current state of the pool.

```rust
pub fn calculate_shares(amount: f64, total_balance: f64, total_shares: f64) -> f64
```

#### Parameters:
- `amount`: The amount of tokens being deposited
- `total_balance`: The total balance of tokens in the pool
- `total_shares`: The total number of shares issued by the pool

#### Return Value:
- The number of shares to be issued to the user

#### Logic:
1. If the pool is empty (`total_shares == 0`), the share price is set to 1
2. Otherwise, the share price is calculated as `total_balance / total_shares`
3. For the first deposit into an empty pool, the shares issued equal the deposit amount
4. For subsequent deposits, shares issued equal `amount / share_price`

#### Examples:

**First Deposit (Empty Pool):**
```
amount = 100
total_balance = 0
total_shares = 0
Result: 100 shares
```

**Subsequent Deposit:**
```
amount = 100
total_balance = 1000
total_shares = 500
share_price = 1000/500 = 2
Result: 100/2 = 50 shares
```

### 2. `calculate_pool_liquidity_amounts`

This function calculates how to optimally distribute tokens when adding liquidity to a pool, determining how much of the input token should be swapped to the second token.

```rust
pub fn calculate_pool_liquidity_amounts(
    amount: f64,
    pool_ratio: f64,
    swap_price: f64,
) -> CalculatePoolLiquidityAmountsResponse
```

#### Parameters:
- `amount`: The total amount of the input token available
- `pool_ratio`: The ratio of token_1 to token_0 in the pool (token_1_price / token_0_price)
- `swap_price`: The exchange rate for swapping token_0 to token_1

#### Return Value:
A struct containing:
- `token_0_for_swap`: Amount of token_0 to swap for token_1
- `token_0_for_pool`: Amount of token_0 to deposit directly into the pool
- `token_1_for_pool`: Amount of token_1 to deposit into the pool (obtained from the swap)

#### Logic:
1. Calculate the optimal amount of token_0 to swap: `amount * pool_ratio / (swap_price + pool_ratio)`
2. Determine the remaining token_0 for direct pool deposit: `amount - token_0_for_swap`
3. Calculate the amount of token_1 received from the swap: `token_0_for_swap * swap_price`
4. Calculate the required token_1 for balanced pool deposit: `token_0_for_pool * pool_ratio`
5. If the swapped amount of token_1 is insufficient, adjust the token_0 amount for direct deposit
6. Return the final distribution of tokens

#### Examples:

**Equal Pool Ratio and Swap Price (1:1):**
```
amount = 1000
pool_ratio = 1.0
swap_price = 1.0
Result:
  token_0_for_swap = 500
  token_0_for_pool = 500
  token_1_for_pool = 500
```

**Different Pool Ratio and Swap Price (2:1):**
```
amount = 1000
pool_ratio = 2.0
swap_price = 2.0
Result:
  token_0_for_swap = 500
  token_0_for_pool = 500
  token_1_for_pool = 1000
```

**Unequal Pool Ratio and Swap Price:**
```
amount = 1000
pool_ratio = 3.0
swap_price = 2.0
Result:
  token_0_for_swap = 600
  token_0_for_pool = 400
  token_1_for_pool = 1200
```

## Practical Application

### Deposit Flow

1. User deposits 1000 ICP into a strategy
2. System identifies the best pool (e.g., ICP_ckUSDT with a 2:1 ratio)
3. `calculate_pool_liquidity_amounts` determines:
   - 500 ICP should be swapped to ckUSDT
   - 500 ICP should be kept for direct deposit
4. After the swap, we have 500 ICP and 1000 ckUSDT
5. These tokens are added to the liquidity pool
6. `calculate_shares` determines how many shares to issue to the user

### Rebalancing Flow

1. System detects a higher APY in a different pool
2. Liquidity is withdrawn from the current pool
3. `calculate_pool_liquidity_amounts` determines the optimal distribution for the new pool
4. Tokens are swapped as needed and deposited into the new pool
5. Total shares remain unchanged, but the underlying assets are now in a higher-yielding pool

## Implementation Notes

- All calculations round to the nearest whole number to avoid dust amounts
- The functions handle edge cases like empty pools and zero balances
- The implementation ensures that the total amount of token_0 used never exceeds the input amount
- The formulas are designed to minimize slippage and maximize capital efficiency
- The calculations account for different pool ratios and swap prices to optimize returns

## Testing

The calculator functions are thoroughly tested with various scenarios:
- Empty pool deposits
- Deposits to existing pools
- Various pool ratios and swap prices
- Edge cases and boundary conditions

These tests ensure the mathematical accuracy and robustness of the calculations that power the AlfaAPY protocol.
